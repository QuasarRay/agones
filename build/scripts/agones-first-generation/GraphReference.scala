// Focused build-time observation of the original pinned graph implementation.
import flatgraph.*
import io.shiftleft.codepropertygraph.generated.{Cpg, GraphSchema}
import java.nio.file.{Files, Paths}
import ujson.*

object GraphReference {
  def main(args: Array[String]): Unit = {
    val graph = Cpg.empty.graph
    def detached(label: String) = new GenericDNode(GraphSchema.getNodeKindByLabel(label).toShort)
    val first = detached("METHOD")
    val second = detached("METHOD")
    val identifier = detached("IDENTIFIER")
    val initial = Cpg.newDiffGraphBuilder
    initial.addNode(first).addNode(second).addEdge(first, identifier, "AST").addNode(first)
    val firstChanges = DiffGraphApplier.applyDiff(graph, initial)
    val a = first.storedRef.get
    val b = second.storedRef.get
    val id = identifier.storedRef.get
    def property(node: GNode, name: String): Value = Arr.from(Accessors.getNodeProperty(node, GraphSchema.getPropertyKindByName(name)).map {
      case s: String => Str(s)
      case n: Int => Num(n)
      case b: Boolean => Bool(b)
      case null => Null
      case other => throw new IllegalArgumentException("unaccounted probe value " + other)
    })
    def snapshot(): Value = Obj(
      "nodes" -> Arr.from(graph.allNodes.map(n => Obj("id" -> n.id().toString, "label" -> n.label(), "name" -> property(n, "NAME"), "order" -> property(n, "ORDER")))),
      "edges" -> Arr.from(graph.allEdges.map(e => Obj("source" -> e.src.id().toString, "target" -> e.dst.id().toString, "label" -> e.label, "property" -> (if (e.property == null) Null else Str(e.property.toString)))))
    )
    val before = snapshot()
    val changes = Cpg.newDiffGraphBuilder
    changes.setNodeProperty(a, "NAME", "alpha")
    changes.setNodeProperty(a, "ORDER", 1)
    changes.setNodeProperty(a, "ORDER", 7)
    changes.addEdge(a, b, "REACHING_DEF")
    changes.addEdge(a, b, "REACHING_DEF", "x")
    changes.addEdge(a, b, "AST")
    val secondChanges = DiffGraphApplier.applyDiff(graph, changes)
    val after = snapshot()
    val reaching = Accessors.getEdgesOut(a, GraphSchema.getEdgeKindByLabel("REACHING_DEF"))
    val deletion = Cpg.newDiffGraphBuilder
    deletion.removeNode(id).addEdge(a, id, "AST")
    deletion.setEdgeProperty(reaching.head, "updated").removeEdge(reaching.last)
    val thirdChanges = DiffGraphApplier.applyDiff(graph, deletion)
    val deleted = snapshot()
    graph.close()
    val closedRejected = try {
      DiffGraphApplier.applyDiff(graph, Cpg.newDiffGraphBuilder)
      false
    } catch { case _: flatgraph.GraphClosedException => true }
    val failingGraph = Cpg.empty.graph
    val removed = detached("METHOD")
    val allocate = Cpg.newDiffGraphBuilder
    allocate.addNode(removed)
    DiffGraphApplier.applyDiff(failingGraph, allocate)
    val failure = Cpg.newDiffGraphBuilder
    failure.setNodeProperty(removed.storedRef.get, "NAME", "deleted").removeNode(removed.storedRef.get)
    val deletedPropertyFailure = try {
      DiffGraphApplier.applyDiff(failingGraph, failure)
      "NO_FAILURE"
    } catch { case error: scala.MatchError => error.getClass.getName }
    val output = Obj("firstChanges" -> firstChanges, "secondChanges" -> secondChanges, "thirdChanges" -> thirdChanges,
      "before" -> before, "after" -> after, "deleted" -> deleted, "closedRejected" -> closedRejected,
      "deletedPropertyFailure" -> deletedPropertyFailure)
    Files.writeString(Paths.get(args(0)), write(output, indent = 2))
  }
}
