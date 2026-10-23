// Owned bootstrap observation. Captures original CpgSchema.instance and Protogen.main.
import flatgraph.schema.SchemaConstructionTrace
import io.shiftleft.codepropertygraph.schema.{CpgSchema, Protogen}
import java.nio.file.{Files, Paths}

object CaptureSchemaConstruction {
  def main(args: Array[String]): Unit = {
    require(args.length == 2, "pass trace output and isolated protobuf output directory")
    SchemaConstructionTrace.start()
    val domain = CpgSchema.instance
    val domainEvents = SchemaConstructionTrace.stop()
    val domainModel = SchemaConstructionTrace.model(domain)
    SchemaConstructionTrace.start()
    Protogen.main(Array(args(1)))
    val protobuf = SchemaConstructionTrace.lastSchema
    val protobufEvents = SchemaConstructionTrace.stop()
    val result = ujson.Obj("formatVersion" -> 1,
      "domain" -> ujson.Obj("events" -> ujson.Arr.from(domainEvents), "schema" -> domainModel),
      "protobuf" -> ujson.Obj("events" -> ujson.Arr.from(protobufEvents), "schema" -> SchemaConstructionTrace.model(protobuf)))
    Files.writeString(Paths.get(args(0)), ujson.write(result, indent = 2) + "\n")
  }
}
