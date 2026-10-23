// Owned build-time instrumentation. This is not a native runtime dependency.
package flatgraph.schema

import java.util.IdentityHashMap
import scala.collection.mutable.ArrayBuffer

object SchemaConstructionTrace {
  private var enabled = false
  private val ids = new IdentityHashMap[AnyRef, String]()
  private val events = ArrayBuffer.empty[ujson.Value]
  private var lastBuilt: Option[Schema] = None

  def start(): Unit = {
    enabled = true
    ids.clear()
    events.clear()
    lastBuilt = None
  }
  def stop(): Seq[ujson.Value] = { enabled = false; events.toSeq }
  def lastSchema: Schema = lastBuilt.getOrElse(throw new IllegalStateException("no schema build was recorded"))

  private lazy val exporterClass = Class.forName("CompleteSchemaExport$")
  private lazy val exporter = exporterClass.getField("MODULE$").get(null)
  private def invokeExporter(name: String, value: Any): ujson.Value = {
    val method = exporterClass.getDeclaredMethods.find(m => m.getName == name && m.getParameterCount == 1)
      .getOrElse(throw new IllegalStateException("missing pinned exporter method: " + name))
    method.setAccessible(true)
    method.invoke(exporter, value.asInstanceOf[AnyRef]).asInstanceOf[ujson.Value]
  }
  def model(schema: Schema): ujson.Value = invokeExporter("schema", schema)
  private def info(value: SchemaInfo): ujson.Value = invokeExporter("info", value)
  private def scalar(value: Any): ujson.Value = invokeExporter("scalar", value)
  private def cardinality(value: Property.Cardinality): ujson.Value = invokeExporter("cardinality", value)
  private def optional(value: String): ujson.Value = Option(value).map(ujson.Str(_)).getOrElse(ujson.Null)
  private def comment(value: Option[String]): ujson.Value = value.map(ujson.Str(_)).getOrElse(ujson.Null)
  private def valueType(value: Property.ValueType[?]): String = value.getClass.getSimpleName.stripSuffix("$")
  private def edgeCard(value: EdgeType.Cardinality): String = value.getClass.getSimpleName.stripSuffix("$")

  private def append(op: String, data: ujson.Obj): Unit = {
    val frames = Thread.currentThread().getStackTrace.toSeq.filter(frame =>
      frame.getClassName.startsWith("io.shiftleft.codepropertygraph.schema.") &&
      frame.getFileName != null && frame.getLineNumber > 0)
    data("op") = op
    data("index") = events.size
    data("sourceFrames") = ujson.Arr.from(frames.map(frame =>
      ujson.Obj("class" -> frame.getClassName, "method" -> frame.getMethodName,
                "file" -> frame.getFileName, "line" -> frame.getLineNumber)))
    events.append(data)
  }
  private def define(value: AnyRef, op: String, data: ujson.Obj): String = {
    require(!ids.containsKey(value), "schema element registered twice")
    val id = "r" + ids.size
    ids.put(value, id)
    data("id") = id
    append(op, data)
    id
  }
  private def ref(value: AnyRef): String = {
    if (ids.containsKey(value)) ids.get(value)
    else value match {
      case constant: Constant[?] =>
        define(value, "constant", ujson.Obj("name" -> constant.name, "value" -> constant.value,
          "valueType" -> valueType(constant.valueType), "comment" -> comment(constant.comment),
          "schemaInfo" -> info(constant.schemaInfo)))
      case _ => throw new IllegalStateException("unregistered schema operand: " + value.getClass.getName)
    }
  }
  private def nullableRef(value: AnyRef): ujson.Value =
    if (value == null) ujson.Null else ujson.Str(ref(value))

  def builder(value: SchemaBuilder, domain: String, pkg: String, traversals: Seq[String]): Unit = if (enabled) {
    define(value, "builder", ujson.Obj("domainShortName" -> domain, "basePackage" -> pkg,
      "additionalTraversalsPackages" -> ujson.Arr.from(traversals)))
  }
  def property(builder: SchemaBuilder, value: Property[?]): Unit = if (enabled) {
    define(value, "property", ujson.Obj("builder" -> ref(builder), "name" -> value.name,
      "valueType" -> valueType(value.valueType), "comment" -> comment(value.comment), "schemaInfo" -> info(value.schemaInfo)))
  }
  def node(value: AbstractNodeType): Unit = if (enabled) {
    val kind = value match { case _: AnyNodeType => "Any"; case _: NodeType => "Concrete"; case _ => "Base" }
    define(value, "node", ujson.Obj("name" -> value.name, "kind" -> kind,
      "comment" -> comment(value.comment), "schemaInfo" -> info(value.schemaInfo)))
  }
  def element(value: AnyRef): Unit = if (enabled) {
    value match {
      case nodeValue: AbstractNodeType => node(nodeValue)
      case edge: EdgeType => define(value, "edge", ujson.Obj("name" -> edge.name,
        "comment" -> comment(edge.comment), "schemaInfo" -> info(edge.schemaInfo)))
      case _ => throw new IllegalStateException("unknown schema element appended to builder")
    }
  }
  def protoId(value: AnyRef, id: Int): Unit = if (enabled) {
    val target = ref(value)
    append("protoId", ujson.Obj("target" -> target, "value" -> id))
  }
  def mandatory(value: Property[?], default: Any): Unit = if (enabled) {
    append("mandatory", ujson.Obj("target" -> ref(value), "value" -> scalar(default)))
  }
  def asList(value: Property[?]): Unit = if (enabled) { append("asList", ujson.Obj("target" -> ref(value))) }
  def starter(value: AbstractNodeType, name: String): Unit = if (enabled) {
    append("starter", ujson.Obj("target" -> ref(value), "value" -> optional(name)))
  }
  def addProperty(value: AbstractNodeType, property: Property[?]): Unit = if (enabled) {
    append("addProperty", ujson.Obj("target" -> ref(value), "property" -> ref(property)))
  }
  def extend(value: AbstractNodeType, bases: Seq[NodeBaseType]): Unit = if (enabled) {
    append("extend", ujson.Obj("target" -> ref(value), "bases" -> ujson.Arr.from(bases.map(base => ref(base)))))
  }
  def marker(value: AbstractNodeType, name: String): Unit = if (enabled) {
    append("marker", ujson.Obj("target" -> ref(value), "name" -> name))
  }
  def primaryKey(value: NodeType, property: Property[?]): Unit = if (enabled) {
    append("primaryKey", ujson.Obj("target" -> ref(value), "property" -> nullableRef(property)))
  }
  def contained(value: NodeType, child: AbstractNodeType, name: String, card: Property.Cardinality, text: String): Unit = if (enabled) {
    append("contained", ujson.Obj("target" -> ref(value), "node" -> ref(child), "name" -> name,
      "cardinality" -> cardinality(card), "comment" -> comment(Helpers.stringToOption(text))))
  }
  def connect(out: AbstractNodeType, edge: EdgeType, in: AbstractNodeType,
    outCard: EdgeType.Cardinality, inCard: EdgeType.Cardinality,
    outStep: String, outDoc: String, inStep: String, inDoc: String): Unit = if (enabled) {
    append("connect", ujson.Obj("out" -> ref(out), "edge" -> ref(edge), "in" -> ref(in),
      "outCardinality" -> edgeCard(outCard), "inCardinality" -> edgeCard(inCard),
      "outStep" -> outStep, "outDoc" -> outDoc, "inStep" -> inStep, "inDoc" -> inDoc))
  }
  def edgeProperty(value: EdgeType, property: Property[?]): Unit = if (enabled) {
    append("edgeProperty", ujson.Obj("target" -> ref(value), "property" -> nullableRef(property)))
  }
  def edgeAccessor(value: EdgeType, name: String): Unit = if (enabled) {
    append("edgeAccessor", ujson.Obj("target" -> ref(value), "value" -> optional(name)))
  }
  def constants(builder: SchemaBuilder, category: String, values: Seq[Constant[?]]): Unit = if (enabled) {
    val references = values.map(value => ref(value))
    append("constants", ujson.Obj("builder" -> ref(builder), "category" -> category, "values" -> ujson.Arr.from(references)))
  }
  def noWarn(builder: SchemaBuilder, node: AbstractNodeType, property: Property[?]): Unit = if (enabled) {
    append("noWarn", ujson.Obj("builder" -> ref(builder), "node" -> ref(node), "property" -> ref(property)))
  }
  def protoOptions(builder: SchemaBuilder, value: ProtoOptions): Unit = if (enabled) {
    val options = if (value == null) ujson.Null else ujson.Obj("package" -> value.pkg,
      "javaOuterClassname" -> value.javaOuterClassname, "javaPackage" -> value.javaPackage,
      "goPackage" -> value.goPackage, "csharpNamespace" -> value.csharpNamespace,
      "uncommonProtoEnumNameMappings" -> ujson.Obj.from(value.uncommonProtoEnumNameMappings.toSeq.sortBy(_._1).map { case (k,v) => k -> ujson.Str(v) }))
    append("protoOptions", ujson.Obj("builder" -> ref(builder), "options" -> options))
  }
  def built(builder: SchemaBuilder, schema: Schema): Unit = if (enabled) {
    lastBuilt = Some(schema)
    append("build", ujson.Obj("builder" -> ref(builder)))
  }
}
