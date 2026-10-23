// Build-time observations call the unmodified pinned CPG loader, mappings and archives.
import io.shiftleft.codepropertygraph.cpgloading.{ProtoCpgLoader, ZipArchive}
import io.shiftleft.codepropertygraph.generated.GraphSchema
import io.shiftleft.proto.cpg.Cpg.*
import io.shiftleft.SerializedCpg
import flatgraph.Accessors
import java.nio.file.{Files, Paths}
import scala.jdk.CollectionConverters.*
import ujson.*

object ProtoLoadingReference {
  def main(args: Array[String]): Unit = {
    val output = Paths.get(args(0))
    val directory = output.getParent
    def string(value: String) = PropertyValue.newBuilder.setStringValue(value).build
    def int(value: Int) = PropertyValue.newBuilder.setIntValue(value).build
    def property(name: NodePropertyName, value: PropertyValue) = CpgStruct.Node.Property.newBuilder.setName(name).setValue(value).build
    def node(key: Long, typ: CpgStruct.Node.NodeType, properties: CpgStruct.Node.Property*) = CpgStruct.Node.newBuilder.setKey(key).setType(typ).addAllProperty(properties.toList.asJava).build
    def edge(src: Long, dst: Long, typ: CpgStruct.Edge.EdgeType) = CpgStruct.Edge.newBuilder.setSrc(src).setDst(dst).setType(typ).build
    def graph(nodes: Seq[CpgStruct.Node], edges: Seq[CpgStruct.Edge] = Seq.empty) = CpgStruct.newBuilder.addAllNode(nodes.asJava).addAllEdge(edges.asJava).build
    def observe(name: String, proto: CpgStruct): Value = {
      Files.write(directory.resolve("first-generation-proto-reference-" + name + ".pb"), proto.toByteArray)
      try {
        val cpg = ProtoCpgLoader.loadFromListOfProtos(java.util.List.of(proto), None)
        val nodes = cpg.graph.allNodes.map { node =>
          val properties = Accessors.getNodeProperties(node).iterator.map { case (name, values) =>
            name -> Arr.from(values.map {
              case s: String => Str(s)
              case n: Int => Num(n)
              case b: Boolean => Bool(b)
              case null => Null
              case other => throw new IllegalArgumentException("unaccounted reference value " + other)
            })
          }.toMap
          Obj("id" -> node.id().toString, "label" -> node.label(), "properties" -> Obj.from(properties))
        }.toList
        val edges = cpg.graph.allEdges.map(e => Obj("src" -> e.src.id().toString, "dst" -> e.dst.id().toString,
          "label" -> e.label, "property" -> (if (e.property == null) Null else Str(e.property.toString)))).toList
        cpg.close()
        Obj("nodes" -> Arr.from(nodes), "edges" -> Arr.from(edges))
      } catch { case error: Exception => Obj("errorClass" -> error.getClass.getName, "errorMessage" -> Option(error.getMessage).getOrElse("")) }
    }
    val a = node(1, CpgStruct.Node.NodeType.TYPE, property(NodePropertyName.FULL_NAME, string("T")), property(NodePropertyName.NAME, string("first")))
    val b = node(2, CpgStruct.Node.NodeType.TYPE, property(NodePropertyName.FULL_NAME, string("T")), property(NodePropertyName.NAME, string("second")))
    val method = node(3, CpgStruct.Node.NodeType.METHOD, property(NodePropertyName.NAME, string("shared")), property(NodePropertyName.ORDER, int(-2)), property(NodePropertyName.IS_EXTERNAL, PropertyValue.newBuilder.setBoolValue(true).build))
    val identifier = node(4, CpgStruct.Node.NodeType.IDENTIFIER, property(NodePropertyName.NAME, string("shared")),
      property(NodePropertyName.DYNAMIC_TYPE_HINT_FULL_NAME, PropertyValue.newBuilder.setStringList(StringList.newBuilder.addValues("T").addValues("T")).build))
    val duplicateTypes = graph(Seq(a, b, method, identifier), Seq(edge(3, 1, CpgStruct.Edge.EdgeType.EVAL_TYPE), edge(3, 2, CpgStruct.Edge.EdgeType.EVAL_TYPE)))
    val unknown = node(99, CpgStruct.Node.NodeType.UNKNOWN_NODE_TYPE)
    val unknownWithProperty = unknown.toBuilder.addProperty(property(NodePropertyName.NAME, string("unknown"))).build
    val twoEdgeProps = edge(3, 3, CpgStruct.Edge.EdgeType.REACHING_DEF).toBuilder
      .addProperty(CpgStruct.Edge.Property.newBuilder.setName(EdgePropertyName.VARIABLE).setValue(string("a")))
      .addProperty(CpgStruct.Edge.Property.newBuilder.setName(EdgePropertyName.VARIABLE).setValue(string("b"))).build
    val cases = Obj(
      "duplicate-types" -> observe("duplicate-types", duplicateTypes),
      "unknown-empty" -> observe("unknown-empty", graph(Seq(method, unknown), Seq(edge(3, 99, CpgStruct.Edge.EdgeType.CALL)))),
      "unknown-with-property" -> observe("unknown-with-property", graph(Seq(unknownWithProperty))),
      "unsupported-long" -> observe("unsupported-long", graph(Seq(node(1, CpgStruct.Node.NodeType.METHOD, property(NodePropertyName.ORDER, PropertyValue.newBuilder.setLongValue(1L).build))))),
      "multiple-edge-properties" -> observe("multiple-edge-properties", graph(Seq(method), Seq(twoEdgeProps))),
      "null-edge-property" -> observe("null-edge-property", graph(Seq(method), Seq(edge(3, 3, CpgStruct.Edge.EdgeType.REACHING_DEF)))),
      "unknown-enum" -> observe("unknown-enum", graph(Seq(CpgStruct.Node.newBuilder.setKey(123).setTypeValue(987654).build)))
    )
    val archive = new ZipArchive(directory.resolve("first-generation-zip-reference-input.zip").toString)
    val entries = archive.entries.map(_.toString)
    val contents = entries.map(name => name -> Files.readAllBytes(archive.getFileEntries.asScala.find(_.toString == name).get).map(b => f"${b & 0xff}%02x").mkString).toMap
    archive.close()
    val compare = ProtoCpgLoader.getClass.getDeclaredMethods.find(_.getName.contains("compareOverlayPath")).get
    compare.setAccessible(true)
    val comparisons = Seq(("/10_a", "/2_b"), ("/2_a", "/2_b"), ("/plain", "/7_"), ("/+2_a", "/2_b"),
      ("/٢_a", "/2_b"), ("/a_b", "/1_c"), ("/2147483648_a", "/1_b"), ("/-2_a", "/1_b"), ("/𐀀", "/\ue000")).map { case (left, right) =>
      val result = Obj("left" -> left, "right" -> right)
      try {
        val less = compare.invoke(ProtoCpgLoader, Paths.get(left), Paths.get(right)).asInstanceOf[Boolean]
        val greater = compare.invoke(ProtoCpgLoader, Paths.get(right), Paths.get(left)).asInstanceOf[Boolean]
        result("comparison") = Num(if (less) -1 else if (greater) 1 else 0)
      } catch { case error: java.lang.reflect.InvocationTargetException => result("errorClass") = Str(error.getCause.getClass.getName) }
      result
    }
    val serializedPath = directory.resolve("first-generation-serialized-reference.zip")
    val overlayA = CpgOverlay.newBuilder.addNode(method).build
    val overlayB = CpgOverlay.newBuilder.addNode(identifier).build
    val dummy = new SerializedCpg
    dummy.addOverlay(overlayA, "dummy")
    val dummyEmpty = dummy.isEmpty
    dummy.close()
    val serialized = new SerializedCpg(serializedPath.toString)
    val createdAtOpen = Files.exists(serializedPath)
    serialized.addOverlay(overlayA, "one")
    serialized.addOverlay(Iterator(overlayA, overlayB), "batch")
    serialized.close()
    val emptyAfterClose = serialized.isEmpty
    val reopened = new SerializedCpg(serializedPath.toString)
    reopened.addOverlay(overlayB, "one")
    reopened.close()
    val result = Obj("cases" -> cases, "archiveEntries" -> Arr.from(entries), "archiveContents" -> Obj.from(contents.map { case (name, value) => name -> Str(value) }),
      "comparisons" -> Arr.from(comparisons), "dummyEmpty" -> dummyEmpty, "createdAtOpen" -> createdAtOpen, "emptyAfterClose" -> emptyAfterClose)
    Files.writeString(output, write(result, indent = 2))
  }
}
