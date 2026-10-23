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
    if (mode == "generate") {
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
