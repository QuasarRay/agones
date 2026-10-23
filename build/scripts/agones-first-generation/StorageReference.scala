// Focused comparisons invoke the pinned native Flatgraph implementation directly.
import flatgraph.*
import flatgraph.storage.{Serialization, Deserialization}
import io.shiftleft.codepropertygraph.generated.{Cpg, GraphSchema}
import java.nio.file.{Files, Paths, Path}
import ujson.*

object StorageReference {
  def scalar(value: Any): Value = value match {
    case null => Obj("kind" -> "Null")
    case value: String => Obj("kind" -> "String", "value" -> value)
    case value: Boolean => Obj("kind" -> "Boolean", "value" -> value)
    case value: Byte => Obj("kind" -> "Byte", "decimal" -> value.toString)
    case value: Short => Obj("kind" -> "Short", "decimal" -> value.toString)
    case value: Int => Obj("kind" -> "Int", "decimal" -> value.toString)
    case value: Long => Obj("kind" -> "Long", "decimal" -> value.toString)
    case value: Float => Obj("kind" -> "Float", "bits" -> f"${java.lang.Float.floatToRawIntBits(value)}%08x")
    case value: Double => Obj("kind" -> "Double", "bits" -> f"${java.lang.Double.doubleToRawLongBits(value)}%016x")
    case value: GNode => Obj("kind" -> "Ref", "id" -> value.id().toString)
    case other => throw new IllegalArgumentException("unaccounted snapshot value " + other)
  }
  def snapshot(graph: Graph): Value = {
    def edge(value: Edge) = Obj("src" -> value.src.id().toString, "dst" -> value.dst.id().toString,
      "label" -> value.label, "property" -> scalar(value.property))
    Obj("nodes" -> Arr.from(graph.allNodes.map { node =>
      Obj("id" -> node.id().toString, "label" -> node.label(),
        "properties" -> Obj.from(Accessors.getNodeProperties(node).iterator.map { case (name, values) => name -> Arr.from(values.map(scalar)) }),
        "out" -> Arr.from(Accessors.getEdgesOut(node).map(edge)), "in" -> Arr.from(Accessors.getEdgesIn(node).map(edge)))
    }))
  }
  def buildCpg(): Graph = {
    val graph = Cpg.empty.graph
    def detached(label: String) = new GenericDNode(GraphSchema.getNodeKindByLabel(label).toShort)
    val a = detached("METHOD"); val b = detached("METHOD"); val id = detached("IDENTIFIER")
    val finding = detached("FINDING"); val dead = detached("LOCAL")
    val raw = Cpg.newDiffGraphBuilder
    Seq(a, b, id, finding, dead).foreach(raw.addNode)
    DiffGraphApplier.applyDiff(graph, raw)
    val aa = a.storedRef.get; val bb = b.storedRef.get; val ii = id.storedRef.get
    val diff = Cpg.newDiffGraphBuilder
    diff.setNodeProperty(aa, "NAME", "same").setNodeProperty(bb, "NAME", "same")
    diff.setNodeProperty(ii, "NAME", "x").setNodeProperty(aa, "ORDER", -2).setNodeProperty(bb, "IS_EXTERNAL", true)
    diff.setNodeProperty(finding.storedRef.get, "evidence", Seq(bb, aa, null))
    diff.setNodeProperty(dead.storedRef.get, "NAME", "removed")
    diff.addEdge(bb, ii, "AST").addEdge(aa, ii, "AST")
    diff.addEdge(aa, bb, "REACHING_DEF").addEdge(aa, bb, "REACHING_DEF", null).addEdge(aa, bb, "REACHING_DEF", "x")
    diff.unsafeAddHalfEdgeForward(bb, ii, GraphSchema.getEdgeKindByLabel("CFG"))
    diff.unsafeAddHalfEdgeBackward(aa, bb, GraphSchema.getEdgeKindByLabel("CFG"))
    diff.addEdge(aa, bb, "CALL")
    DiffGraphApplier.applyDiff(graph, diff)
    val remove = Cpg.newDiffGraphBuilder
    remove.removeNode(dead.storedRef.get).removeEdge(Accessors.getEdgesOut(aa, GraphSchema.getEdgeKindByLabel("CALL")).head)
    DiffGraphApplier.applyDiff(graph, remove)
    graph
  }
  def buildPrimitive(): Graph = {
    val labels = Array("BOOL", "BYTE", "SHORT", "INT", "LONG", "FLOAT", "DOUBLE", "STR", "REF")
    val prototypes = Array[AnyRef](Array.empty[Boolean], Array.empty[Byte], Array.empty[Short], Array.empty[Int], Array.empty[Long],
      Array.empty[Float], Array.empty[Double], Array.empty[String], Array.empty[GNode])
    val schema = new FreeSchema(Array("N"), labels, prototypes, Map("N" -> labels.toSet), Array("X"), Array[AnyRef](Array(0d)))
    val graph = new Graph(schema)
    val a = new GenericDNode(0); val b = new GenericDNode(0)
    val raw = new DiffGraphBuilder(schema); raw.addNode(a).addNode(b); DiffGraphApplier.applyDiff(graph, raw)
    val node = a.storedRef.get
    val values = Array[Any](Seq(true, false), Seq(Byte.MinValue, Byte.MaxValue), Seq(Short.MinValue, Short.MaxValue), Seq(Int.MinValue, Int.MaxValue),
      Seq(Long.MinValue, Long.MaxValue), Seq(java.lang.Float.intBitsToFloat(0x7fc01234), -0f),
      Seq(java.lang.Double.longBitsToDouble(0x7ff8000000001234L), -0d), Seq("same", null, "😀"), Seq(b.storedRef.get, null))
    val diff = new DiffGraphBuilder(schema)
    labels.zip(values).foreach { case (name, value) => diff.setNodeProperty(node, name, value) }
    diff.addEdge(node, b.storedRef.get, "X", java.lang.Double.longBitsToDouble(0x7ff8000000001234L))
    DiffGraphApplier.applyDiff(graph, diff)
    graph
  }
  def main(args: Array[String]): Unit = {
    val mode = args(0); val output = Paths.get(args(1))
    if (mode == "generate-legacy-metadata") {
      val result=Obj()
      for ((name,key) <- Seq("library"->"LIBRARY_VERSIONS_MAX_ID","strings"->"STRING_TO_INT_MAX_ID")) {
        val path=output.resolveSibling(s"first-generation-legacy-meta-$name.odb");require(!Files.exists(path))
        Files.copy(output.resolveSibling("first-generation-legacy-reference.odb"),path)
        val store=new org.h2.mvstore.MVStore.Builder().fileName(path.toString).open()
        val meta=store.openMap[String,String]("metadata");meta.put(key,"invalid")
        if (name=="library") meta.put("STORAGE_FORMAT_VERSION","3")
        store.close()
        try {flatgraph.convert.Convert.convertOdbToFlatgraph(path,output.resolveSibling(s"first-generation-legacy-meta-$name.fg"));result(name)=Obj("unexpectedSuccess"->true)}
        catch {case error:Throwable=>result(name)=Obj("error"->error.getClass.getName)}
      }
      Files.writeString(output,write(result,indent=2))
    } else if (mode == "generate-legacy-modes") {
      val result=Obj()
      for (mode <- Seq("plain","deflate","empty")) {
        val path=output.resolveSibling(s"first-generation-legacy-mode-$mode.odb");require(!Files.exists(path))
        val builder=new org.h2.mvstore.MVStore.Builder().fileName(path.toString).autoCommitDisabled()
        if (mode=="deflate") builder.compressHigh()
        val store=builder.open()
        val meta=store.openMap[String,String]("metadata");meta.put("STORAGE_FORMAT_VERSION","2");meta.put("STRING_TO_INT_MAX_ID","2")
        val dictionary=store.openMap[String,java.lang.Integer]("stringToIntMappings");dictionary.put("METHOD",1);dictionary.put("NAME",2)
        val nodes=store.openMap[java.lang.Long,Array[Byte]]("nodes")
        if (mode!="empty") for (id <- 0 until 3) {
          val packer=org.msgpack.core.MessagePack.newDefaultBufferPacker()
          packer.packLong(id);packer.packInt(1);packer.packMapHeader(1);packer.packInt(2);packer.packArrayHeader(2);packer.packByte(1)
          packer.packString(s"$mode-$id-"+"large-value-λ-😀"*3000);packer.packInt(0);packer.packInt(0);nodes.put(id.toLong,packer.toByteArray)
        }
        store.close()
        val target=output.resolveSibling(s"first-generation-legacy-mode-$mode.fg")
        flatgraph.convert.Convert.convertOdbToFlatgraph(path,target)
        val cpg=Cpg.withStorage(target,false);result(mode)=snapshot(cpg.graph);cpg.close()
      }
      Files.writeString(output,write(result,indent=2))
    } else if (mode == "generate-legacy-failures") {
      val result=Obj()
      for ((name,typ) <- Seq("primitive-array"->14,"character"->11,"version"->1,"null-only"->10)) {
        val path=output.resolveSibling(s"first-generation-legacy-failure-$name.odb")
        require(!Files.exists(path))
        val storage=overflowdb.storage.OdbStorage.createWithSpecificLocation(path.toFile,new overflowdb.util.StringInterner)
        val label=storage.lookupOrCreateStringToIntMapping("METHOD");val prop=storage.lookupOrCreateStringToIntMapping("NAME")
        val packer=org.msgpack.core.MessagePack.newDefaultBufferPacker()
        packer.packLong(42);packer.packInt(label);packer.packMapHeader(1);packer.packInt(prop);packer.packArrayHeader(2);packer.packByte(typ.toByte)
        if (typ==14) {packer.packArrayHeader(2);packer.packInt(1);packer.packInt(2)}
        else if (typ==11) packer.packInt(65)
        else if (typ==10) packer.packNil()
        else packer.packString("valid")
        packer.packInt(0);packer.packInt(0);storage.persist(42,packer.toByteArray);storage.flush()
        if (name=="version") {storage.getMetaDataMVMap.put("STORAGE_FORMAT_VERSION","3");storage.getMetaDataMVMap.getStore.close()}
        else storage.close()
        val target=output.resolveSibling(s"first-generation-legacy-failure-$name.fg")
        try {
          flatgraph.convert.Convert.convertOdbToFlatgraph(path,target)
          val cpg=Cpg.withStorage(target,false);result(name)=Obj("snapshot"->snapshot(cpg.graph));cpg.close()
        } catch {case error:Throwable=>result(name)=Obj("error"->error.getClass.getName,"message"->Option(error.getMessage).getOrElse(""))}
      }
      Files.writeString(output,write(result,indent=2))
    } else if (mode == "generate-legacy") {
      val graph=buildCpg()
      val fg=output.resolveSibling("first-generation-legacy-input.fg")
      val odb=output.resolveSibling("first-generation-legacy-reference.odb")
      val converted=output.resolveSibling("first-generation-legacy-converted.fg")
      Serialization.writeGraph(graph,fg)
      flatgraph.convert.Convert.convertFlatgraphToOdb(fg,odb)
      flatgraph.convert.Convert.convertOdbToFlatgraph(odb,converted)
      val cpg=Cpg.withStorage(converted,false)
      val stressPath=output.resolveSibling("first-generation-legacy-multipage.odb")
      val storage=overflowdb.storage.OdbStorage.createWithSpecificLocation(stressPath.toFile,new overflowdb.util.StringInterner)
      val labelId=storage.lookupOrCreateStringToIntMapping("METHOD")
      val nameId=storage.lookupOrCreateStringToIntMapping("NAME")
      def raw(id:Long,name:String):Array[Byte] = {
        val packer=org.msgpack.core.MessagePack.newDefaultBufferPacker()
        packer.packLong(id);packer.packInt(labelId);packer.packMapHeader(1);packer.packInt(nameId)
        packer.packArrayHeader(2);packer.packByte(1);packer.packString(name);packer.packInt(0);packer.packInt(0)
        packer.toByteArray
      }
      for (id <- (0 until 1800).reverse) storage.persist(id.toLong-900,raw(id.toLong-900,s"method-$id-"+"same"*32+"-λ-😀"))
      storage.flush()
      for (id <- 0 until 1800 if id%7==0) storage.removeNode((id.toLong-900):java.lang.Long)
      for (id <- 0 until 1800 if id%11==0 && id%7!=0) storage.persist(id.toLong-900,raw(id.toLong-900,s"updated-$id"))
      storage.close()
      val stressConverted=output.resolveSibling("first-generation-legacy-multipage-converted.fg")
      flatgraph.convert.Convert.convertOdbToFlatgraph(stressPath,stressConverted)
      val stress=Cpg.withStorage(stressConverted,false)
      Files.writeString(output,write(Obj("cpg"->snapshot(cpg.graph),"multipage"->snapshot(stress.graph)),indent=2))
      cpg.close();stress.close();graph.close()
    } else if (mode == "generate") {
      val cpg = buildCpg(); val primitive = buildPrimitive()
      val cpgPath = output.resolveSibling("first-generation-flatgraph-reference.fg")
      val primitivePath = output.resolveSibling("first-generation-flatgraph-primitives.fg")
      val counts = Serialization.writeGraph(cpg, cpgPath)
      Serialization.writeGraph(primitive, primitivePath)
      val reopened = Cpg.withStorage(cpgPath, false)
      val free = Deserialization.readGraph(primitivePath, None, false)
      val result = Obj("cpg" -> snapshot(cpg), "cpgReopened" -> snapshot(reopened.graph), "primitive" -> snapshot(primitive), "primitiveReopened" -> snapshot(free),
        "counts" -> Obj("nodeKinds" -> counts.nodeKinds, "edgeKinds" -> counts.edgeKinds, "propertyKinds" -> counts.propertyKinds))
      reopened.close(); free.close(); cpg.close(); primitive.close()
      Files.writeString(output, write(result, indent = 2))
    } else if (mode == "inspect") {
      val path = Paths.get(args(2)); val cpg = Cpg.withStorage(path, false)
      Files.writeString(output, write(snapshot(cpg.graph), indent = 2)); cpg.close()
    } else if (mode == "inspect-free") {
      val graph = Deserialization.readGraph(Paths.get(args(2)), None, false)
      Files.writeString(output, write(snapshot(graph), indent = 2)); graph.close()
    } else throw new IllegalArgumentException("unknown comparison mode " + mode)
  }
}
