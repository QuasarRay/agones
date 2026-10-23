// Generated native CPG schema construction. Do not edit.
use crate::schema::*;use crate::Result;pub fn populate(builder:&mut SchemaBuilder)->Result<()> {let info_0=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Base$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Base".to_owned(),schema_name:"Base".to_owned(), doc_index:2147483647,description:"\n".to_owned(),provided_by_frontend:true};let info_1=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.FileSystem$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.FileSystem".to_owned(),schema_name:"FileSystem".to_owned(), doc_index:3,description:"\nCPGs are created from sets of files and the File System Layer describes the layout of\nthese files, that is, it provides information about source files\nand shared objects for source-based and machine-code-based frontends\nrespectively. The purpose of including this information in the CPG\nis to allow nodes of the graph to be mapped back to file system locations.\n".to_owned(),provided_by_frontend:false};let info_2=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Namespace$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Namespace".to_owned(),schema_name:"Namespace".to_owned(), doc_index:4,description:"\nMany programming languages allow code to be structured into namespaces. The Namespace\nLayer makes these namespaces explicit and associates program constructs with the\nnamespaces they are defined in.\n".to_owned(),provided_by_frontend:false};let info_3=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Operators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Operators".to_owned(),schema_name:"Operators".to_owned(), doc_index:17,description:"\n\n".to_owned(),provided_by_frontend:false};let info_4=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Configuration$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Configuration".to_owned(),schema_name:"Configuration".to_owned(), doc_index:18,description:"\nThe code property graph specification currently does not contain\nschema elements for the representation of configuration files in\na structured format, however, it does allow configuration files\nto be included verbatim in the graph to enable language-/framework-\nspecific passes to access them. This layer provides the necessary\nschema elements for this basic support of configuration files.\n".to_owned(),provided_by_frontend:false};let info_5=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.MetaData$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.MetaData".to_owned(),schema_name:"MetaData".to_owned(), doc_index:2,description:"\nThe Meta Data Layer contains information about CPG creation. In particular,\nit indicates which language frontend generated the CPG and which overlays\nhave been applied. The layer consists of a single node - the Meta Data node -\nand language frontends MUST create this node. Overlay creators MUST edit\nthis node to indicate that a layer has been successfully applied in all\ncases where applying the layer more than once is prohibitive.\n".to_owned(),provided_by_frontend:true};let info_6=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Type$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Type".to_owned(),schema_name:"Type".to_owned(), doc_index:6,description:"\nThe Type Layer contains information about type declarations, relations between\ntypes, and type instantiation and usage. In its current form, it allows modelling of\nparametrized types, type hierarchies and aliases.\n".to_owned(),provided_by_frontend:true};let info_7=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Method$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Method".to_owned(),schema_name:"Method".to_owned(), doc_index:5,description:"\nThe Method Layer contains declarations of methods, functions, and procedures.\nInput parameters and output parameters (including return parameters) are\nrepresented, however, method contents is not present in this layer.\n".to_owned(),provided_by_frontend:true};let info_8=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Ast$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Ast".to_owned(),schema_name:"Ast".to_owned(), doc_index:7,description:"\nThe Abstract Syntax Tree (AST) Layer provides syntax trees for all compilation units.\nAll nodes of the tree inherit from the same base class (`AST_NODE`) and are connected\nto their child nodes via outgoing `AST` edges.\n\nSyntax trees are typed, that is, when possible, types for all expressions are stored\nin the tree. Moreover, common control structure types are defined in the specification,\nmaking it possible to translate trees into corresponding control flow graphs if only\nthese common control structure types are used, possibly by desugaring on the side of\nthe language frontend. For cases where this is not an option,\nthe AST specification provides means of storing language-dependent information in the\nAST that can be interpreted by language-dependent control flow construction passes.\n\nThis layer MUST be created by the frontend.\n\n".to_owned(),provided_by_frontend:true};let info_9=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.CallGraph$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.CallGraph".to_owned(),schema_name:"CallGraph".to_owned(), doc_index:8,description:"\nThe Call Graph Layer represents call relations between methods.\n".to_owned(),provided_by_frontend:false};let info_10=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Cfg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Cfg".to_owned(),schema_name:"Cfg".to_owned(), doc_index:9,description:"\nThe Control Flow Graph Layer provides control flow graphs for each method. Control\nflow graphs are constructed by marking a sub set of the abstract syntax tree nodes\nas control flow nodes (`CFG_NODE`) and connecting these nodes via `CFG` edges.\nThe control flow graph models both the control flow within the calculation of an\nexpression as well as from expression to expression. The layer can be automatically\ngenerated from the syntax tree layer if only control structure types supported by\nthis specification are employed.\n".to_owned(),provided_by_frontend:false};let info_11=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Dominators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Dominators".to_owned(),schema_name:"Dominators".to_owned(), doc_index:10,description:"\nThe Dominators Layer provides dominator- and post-dominator trees for all methods.\nIt is constructed automatically from the control flow graph layer and is in turn\nused to automatically construct control dependence relations of the PDG layer.\n".to_owned(),provided_by_frontend:false};let info_12=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Pdg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Pdg".to_owned(),schema_name:"Pdg".to_owned(), doc_index:11,description:"\n      The Program Dependence Graph Layer contains a program dependence graph for\neach method of the source program. A program dependence graph consists\nof a data dependence graph (DDG) and a control dependence graph (CDG),\ncreated by connecting nodes of the control flow graph via `REACHING_DEF`\nand `CDG` edges respectively.\n".to_owned(),provided_by_frontend:false};let info_13=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Shortcuts$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Shortcuts".to_owned(),schema_name:"Shortcuts".to_owned(), doc_index:16,description:"\nThe Shortcuts Layer provides shortcut edges calculated to speed up\nsubsequent queries. Language frontends MUST NOT create shortcut edges.\n".to_owned(),provided_by_frontend:false};let info_14=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Comment$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Comment".to_owned(),schema_name:"Comment".to_owned(), doc_index:12,description:"".to_owned(),provided_by_frontend:true};let info_15=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Tags$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Tags".to_owned(),schema_name:"Tags".to_owned(), doc_index:17,description:"\nThe Code Property Graph specification allows for tags to be attached\nto arbitrary nodes. Conceptually, this is similar to the creation of\nFinding nodes, however, tags are to be used for intermediate results\nrather than end-results that are to be reported to the user.\n".to_owned(),provided_by_frontend:false};let info_16=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Binding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Binding".to_owned(),schema_name:"Binding".to_owned(), doc_index:19,description:"\nWe use the concept of \"bindings\" to support resolving of\n(method-name, signature) pairs at type declarations (`TYPE_DECL`). For each\npair that we can resolve, we create a `BINDING` node that is connected the\nthe type declaration via an incoming `BINDS` edge. The `BINDING` node is\nconnected to the method it resolves to via an outgoing `REF` edge.\n".to_owned(),provided_by_frontend:false};let info_17=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Annotation$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Annotation".to_owned(),schema_name:"Annotation".to_owned(), doc_index:20,description:"\nJava Annotation related CPG definitions.\n".to_owned(),provided_by_frontend:true};let info_18=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Finding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Finding".to_owned(),schema_name:"Finding".to_owned(), doc_index:15,description:"\nWe allow findings (e.g., potential vulnerabilities, notes on dangerous practices) to\nbe stored in the Findings Layer.\n".to_owned(),provided_by_frontend:false};let info_19=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Hidden$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Hidden".to_owned(),schema_name:"Hidden".to_owned(), doc_index:-1,description:"\nThe schema elements defined here are NOT included on the schema website.\nThese are schema elements that we are using but would like to rework\nin the future and not make part of the standard.\n".to_owned(),provided_by_frontend:false};let info_20=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize".to_owned(),schema_name:"ProtoSerialize".to_owned(), doc_index:2147483647,description:"\n\n".to_owned(),provided_by_frontend:false};
// CPG_CONSTRUCTION_OP:1
builder.add_property("VERSION".to_owned(),ValueType::String,"A version, given as a string. Used, for example, in the META_DATA node to\nindicate which version of the CPG spec this CPG conforms to",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:2
builder.property_mut("VERSION")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:3
builder.property_mut("VERSION")?.proto_id=Some(13);
// CPG_CONSTRUCTION_OP:4
builder.add_property("HASH".to_owned(),ValueType::String,"This property contains a hash value in the form of a string.\nHashes can be used to summarize data, e.g., to summarize the\ncontents of source files or sub graphs. Such summaries are useful\nto determine whether code has already been analyzed in incremental\nanalysis pipelines. This property is optional to allow its calculation\nto be deferred or skipped if the hash is not needed.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:5
builder.property_mut("HASH")?.proto_id=Some(120);
// CPG_CONSTRUCTION_OP:6
builder.add_property("CODE".to_owned(),ValueType::String,"This field holds the code snippet that the node represents.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:7
builder.property_mut("CODE")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:8
builder.property_mut("CODE")?.proto_id=Some(21);
// CPG_CONSTRUCTION_OP:9
builder.add_property("IS_EXTERNAL".to_owned(),ValueType::Boolean,"Indicates that the construct (METHOD or TYPE_DECL) is external, that is,\nit is referenced but not defined in the code (applies both to insular\nparsing and to library functions where we have header files only)",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:10
builder.property_mut("IS_EXTERNAL")?.mandatory(Scalar::Boolean { value:false })?;
// CPG_CONSTRUCTION_OP:11
builder.property_mut("IS_EXTERNAL")?.proto_id=Some(7);
// CPG_CONSTRUCTION_OP:12
builder.add_property("INDEX".to_owned(),ValueType::Int,"Specifies an index, e.g., for a parameter or argument.\n Explicit parameters are numbered from 1 to N, while index 0 is reserved for implicit\n self / this parameter.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:13
builder.property_mut("INDEX")?.mandatory(Scalar::Int { decimal:"-2".to_owned() })?;
// CPG_CONSTRUCTION_OP:14
builder.property_mut("INDEX")?.proto_id=Some(2223);
// CPG_CONSTRUCTION_OP:15
builder.add_property("NAME".to_owned(),ValueType::String,"Name of represented object, e.g., method name (e.g. \"run\")",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:16
builder.property_mut("NAME")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:17
builder.property_mut("NAME")?.proto_id=Some(5);
// CPG_CONSTRUCTION_OP:18
builder.add_property("FULL_NAME".to_owned(),ValueType::String,"This is the fully-qualified name of an entity, e.g., the fully-qualified\nname of a method or type. The details of what constitutes a fully-qualified\nname are language specific. This field SHOULD be human readable.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:19
builder.property_mut("FULL_NAME")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:20
builder.property_mut("FULL_NAME")?.proto_id=Some(6);
// CPG_CONSTRUCTION_OP:21
builder.add_property("PARSER_TYPE_NAME".to_owned(),ValueType::String,"AST node type name emitted by parser.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:22
builder.property_mut("PARSER_TYPE_NAME")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:23
builder.property_mut("PARSER_TYPE_NAME")?.proto_id=Some(3);
// CPG_CONSTRUCTION_OP:24
builder.add_property("VALUE".to_owned(),ValueType::String,"This property denotes a string value as used in a key-value pair.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:25
builder.property_mut("VALUE")?.mandatory(Scalar::String { value:"".to_owned() })?;
// CPG_CONSTRUCTION_OP:26
builder.property_mut("VALUE")?.proto_id=Some(8);
// CPG_CONSTRUCTION_OP:27
builder.add_property("CONTENT".to_owned(),ValueType::String,"Certain files, e.g., configuration files, may be included in the CPG as-is.\nFor such files, the `CONTENT` field contains the files content.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:28
builder.property_mut("CONTENT")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:29
builder.property_mut("CONTENT")?.proto_id=Some(20);
// CPG_CONSTRUCTION_OP:30
builder.add_property("AST_PARENT_TYPE".to_owned(),ValueType::String,"The type of the AST parent. Since this is only used in some parts of the graph,\nthe list does not include all possible parents by intention.\nPossible parents: METHOD, TYPE_DECL, NAMESPACE_BLOCK.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:31
builder.property_mut("AST_PARENT_TYPE")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:32
builder.property_mut("AST_PARENT_TYPE")?.proto_id=Some(56);
// CPG_CONSTRUCTION_OP:33
builder.add_property("AST_PARENT_FULL_NAME".to_owned(),ValueType::String,"This field holds the FULL_NAME of the AST parent of an entity.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:34
builder.property_mut("AST_PARENT_FULL_NAME")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:35
builder.property_mut("AST_PARENT_FULL_NAME")?.proto_id=Some(57);
// CPG_CONSTRUCTION_OP:36
builder.add_node_base_type("DECLARATION".to_owned(),"This is the base node class for all declarations.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:37
builder.node_mut("DECLARATION")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:38
builder.add_edge_type("REF".to_owned(),"This edge indicates that the source node is an identifier that denotes\naccess to the destination node. For example, an identifier may reference\na local variable.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:39
builder.edge_mut("REF")?.proto_id=Some(10);
// CPG_CONSTRUCTION_OP:40
builder.add_property("FILENAME".to_owned(),ValueType::String,"The path of the source file this node was generated from, relative to the root\npath in the meta data node. This field must be set but may be set to the value `<unknown>` to\nindicate that no source file can be associated with the node, e.g., because the node represents\nan entity known to exist because it is referenced, but for which the file that is is declared in\nis unknown.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:41
builder.property_mut("FILENAME")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:42
builder.property_mut("FILENAME")?.proto_id=Some(106);
// CPG_CONSTRUCTION_OP:43
builder.add_property("LINE_NUMBER".to_owned(),ValueType::Int,"This optional field provides the line number of the program construct\nrepresented by the node.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:44
builder.property_mut("LINE_NUMBER")?.proto_id=Some(2);
// CPG_CONSTRUCTION_OP:45
builder.add_property("COLUMN_NUMBER".to_owned(),ValueType::Int,"This optional fields provides the column number of the program construct\nrepresented by the node.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:46
builder.property_mut("COLUMN_NUMBER")?.proto_id=Some(11);
// CPG_CONSTRUCTION_OP:47
builder.add_property("LINE_NUMBER_END".to_owned(),ValueType::Int,"This optional fields provides the line number at which the program construct\nrepresented by the node ends.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:48
builder.property_mut("LINE_NUMBER_END")?.proto_id=Some(12);
// CPG_CONSTRUCTION_OP:49
builder.add_property("COLUMN_NUMBER_END".to_owned(),ValueType::Int,"This optional fields provides the column number at which the program construct\nrepresented by the node ends.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:50
builder.property_mut("COLUMN_NUMBER_END")?.proto_id=Some(16);
// CPG_CONSTRUCTION_OP:51
builder.add_edge_type("SOURCE_FILE".to_owned(),"This edge connects a node to the node that represents its source file. These\nedges MUST not be created by the language frontend but are automatically\ncreated based on `FILENAME` fields.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:52
builder.edge_mut("SOURCE_FILE")?.proto_id=Some(157);
// CPG_CONSTRUCTION_OP:53
builder.add_node_type("FILE".to_owned(),"File nodes represent source files or a shared objects from which the CPG\nwas generated. File nodes serve as indices, that is, they allow looking up all\nelements of the code by file.\n\nFor each file, the graph CAN contain exactly one File node, if not File nodes\nare created as indicated by `FILENAME` property of other nodes.\nAs file nodes are root nodes of abstract syntax tress, they are AstNodes and\ntheir order field is set to 0. This is because they have no sibling nodes,\nnot because they are the first node of the AST.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:54
builder.node_mut("FILE")?.proto_id=Some(38);
// CPG_CONSTRUCTION_OP:55
builder.node_mut("FILE")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:56
builder.node_mut("FILE")?.add_property("HASH".to_owned());
// CPG_CONSTRUCTION_OP:57
builder.node_mut("FILE")?.add_property("CONTENT".to_owned());
// CPG_CONSTRUCTION_OP:58
builder.node_mut("FILE")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:59
builder.add_property("OFFSET".to_owned(),ValueType::Int,"Start offset into the CONTENT property of the corresponding FILE node.\nThe offset is such that parts of the content can easily\nbe accessed via `content.substring(offset, offsetEnd)`.\nThis means that the offset must be measured in utf16 encoding (i.e. neither in\ncharacters/codeunits nor in byte-offsets into a utf8 encoding).\nE.g. for METHOD nodes this start offset points to the start of the methods\nsource code in the string holding the source code of the entire file.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:60
builder.property_mut("OFFSET")?.proto_id=Some(3812);
// CPG_CONSTRUCTION_OP:61
builder.add_property("OFFSET_END".to_owned(),ValueType::Int,"End offset (exclusive) into the CONTENT property of the corresponding FILE node.\nSee OFFSET documentation for finer details.\nE.g. for METHOD nodes this end offset points to the first code position which is\nnot part of the method.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:62
builder.property_mut("OFFSET_END")?.proto_id=Some(3813);
// CPG_CONSTRUCTION_OP:63
builder.add_node_type("NAMESPACE_BLOCK".to_owned(),"A reference to a namespace.\nWe borrow the concept of a \"namespace block\" from C++, that is, a namespace block\nis a block of code that has been placed in the same namespace by a programmer.\nThis block may be introduced via a `package` statement in Java or\na `namespace{ }` statement in C++.\n\nThe `FULL_NAME` field contains a unique identifier to represent the namespace block\nitself not just the namespace it references. So in addition to the namespace name\nit can be useful to use the containing file name to derive a unique identifier.\n\nThe `NAME` field contains the namespace name in a human-readable format.\nThe name should be given in dot-separated form where a dot indicates\nthat the right hand side is a sub namespace of the left hand side, e.g.,\n`foo.bar` denotes the namespace `bar` contained in the namespace `foo`.",Some(info_2.clone()));
// CPG_CONSTRUCTION_OP:64
builder.node_mut("NAMESPACE_BLOCK")?.proto_id=Some(41);
// CPG_CONSTRUCTION_OP:65
builder.node_mut("NAMESPACE_BLOCK")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:66
builder.node_mut("NAMESPACE_BLOCK")?.add_property("FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:67
builder.node_mut("NAMESPACE_BLOCK")?.add_property("FILENAME".to_owned());
// CPG_CONSTRUCTION_OP:68
builder.node_mut("NAMESPACE_BLOCK")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:69
builder.add_node_type("NAMESPACE".to_owned(),"This node represents a namespace. Similar to FILE nodes, NAMESPACE nodes\nserve as indices that allow all definitions inside a namespace to be\nobtained by following outgoing edges from a NAMESPACE node.\n\nNAMESPACE nodes MUST NOT be created by language frontends. Instead,\nthey are generated from NAMESPACE_BLOCK nodes automatically upon\nfirst loading of the CPG.",Some(info_2.clone()));
// CPG_CONSTRUCTION_OP:70
builder.node_mut("NAMESPACE")?.proto_id=Some(40);
// CPG_CONSTRUCTION_OP:71
builder.node_mut("NAMESPACE")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:72
builder.node_mut("NAMESPACE")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:73
builder.connect("SOURCE_FILE","NAMESPACE_BLOCK","FILE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"namespaceBlock".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:74
let constant_26=Constant{name:"addition".to_owned(),value:"<operator>.addition".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:75
let constant_27=Constant{name:"subtraction".to_owned(),value:"<operator>.subtraction".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:76
let constant_28=Constant{name:"multiplication".to_owned(),value:"<operator>.multiplication".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:77
let constant_29=Constant{name:"division".to_owned(),value:"<operator>.division".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:78
let constant_30=Constant{name:"exponentiation".to_owned(),value:"<operator>.exponentiation".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:79
let constant_31=Constant{name:"modulo".to_owned(),value:"<operator>.modulo".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:80
let constant_32=Constant{name:"shiftLeft".to_owned(),value:"<operator>.shiftLeft".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:81
let constant_33=Constant{name:"logicalShiftRight".to_owned(),value:"<operator>.logicalShiftRight".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Shift right padding with zeros".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:82
let constant_34=Constant{name:"arithmeticShiftRight".to_owned(),value:"<operator>.arithmeticShiftRight".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Shift right preserving the sign".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:83
let constant_35=Constant{name:"not".to_owned(),value:"<operator>.not".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:84
let constant_36=Constant{name:"and".to_owned(),value:"<operator>.and".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:85
let constant_37=Constant{name:"or".to_owned(),value:"<operator>.or".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:86
let constant_38=Constant{name:"xor".to_owned(),value:"<operator>.xor".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:87
let constant_39=Constant{name:"assignmentPlus".to_owned(),value:"<operator>.assignmentPlus".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:88
let constant_40=Constant{name:"assignmentMinus".to_owned(),value:"<operator>.assignmentMinus".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:89
let constant_41=Constant{name:"assignmentMultiplication".to_owned(),value:"<operator>.assignmentMultiplication".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:90
let constant_42=Constant{name:"assignmentDivision".to_owned(),value:"<operator>.assignmentDivision".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:91
let constant_43=Constant{name:"assignmentExponentiation".to_owned(),value:"<operators>.assignmentExponentiation".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:92
let constant_44=Constant{name:"assignmentModulo".to_owned(),value:"<operators>.assignmentModulo".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:93
let constant_45=Constant{name:"assignmentShiftLeft".to_owned(),value:"<operators>.assignmentShiftLeft".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:94
let constant_46=Constant{name:"assignmentLogicalShiftRight".to_owned(),value:"<operators>.assignmentLogicalShiftRight".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:95
let constant_47=Constant{name:"assignmentArithmeticShiftRight".to_owned(),value:"<operators>.assignmentArithmeticShiftRight".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:96
let constant_48=Constant{name:"assignmentAnd".to_owned(),value:"<operators>.assignmentAnd".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:97
let constant_49=Constant{name:"assignmentOr".to_owned(),value:"<operators>.assignmentOr".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:98
let constant_50=Constant{name:"assignmentXor".to_owned(),value:"<operators>.assignmentXor".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:99
let constant_51=Constant{name:"arrayInitializer".to_owned(),value:"<operator>.arrayInitializer".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:100
let constant_52=Constant{name:"assignment".to_owned(),value:"<operator>.assignment".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:101
let constant_53=Constant{name:"minus".to_owned(),value:"<operator>.minus".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("E.g. `a = -b`".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:102
let constant_54=Constant{name:"plus".to_owned(),value:"<operator>.plus".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("E.g. `a = +b`".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:103
let constant_55=Constant{name:"preIncrement".to_owned(),value:"<operator>.preIncrement".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:104
let constant_56=Constant{name:"preDecrement".to_owned(),value:"<operator>.preDecrement".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:105
let constant_57=Constant{name:"postIncrement".to_owned(),value:"<operator>.postIncrement".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:106
let constant_58=Constant{name:"postDecrement".to_owned(),value:"<operator>.postDecrement".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:107
let constant_59=Constant{name:"logicalNot".to_owned(),value:"<operator>.logicalNot".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:108
let constant_60=Constant{name:"logicalOr".to_owned(),value:"<operator>.logicalOr".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:109
let constant_61=Constant{name:"logicalAnd".to_owned(),value:"<operator>.logicalAnd".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:110
let constant_62=Constant{name:"equals".to_owned(),value:"<operator>.equals".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:111
let constant_63=Constant{name:"notEquals".to_owned(),value:"<operator>.notEquals".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:112
let constant_64=Constant{name:"greaterThan".to_owned(),value:"<operator>.greaterThan".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:113
let constant_65=Constant{name:"lessThan".to_owned(),value:"<operator>.lessThan".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:114
let constant_66=Constant{name:"greaterEqualsThan".to_owned(),value:"<operator>.greaterEqualsThan".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:115
let constant_67=Constant{name:"lessEqualsThan".to_owned(),value:"<operator>.lessEqualsThan".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:116
let constant_68=Constant{name:"instanceOf".to_owned(),value:"<operator>.instanceOf".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:117
let constant_69=Constant{name:"memberAccess".to_owned(),value:"<operator>.memberAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deprecated. Formerly in C: `a.b` but not! in Java".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:118
let constant_70=Constant{name:"indirectMemberAccess".to_owned(),value:"<operator>.indirectMemberAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deprecated. Formerly in C: `a->b` and `a.b` in Java".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:119
let constant_71=Constant{name:"computedMemberAccess".to_owned(),value:"<operator>.computedMemberAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deprecated. Formerly in C: `a[b]` but not! in Java".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:120
let constant_72=Constant{name:"indirectComputedMemberAccess".to_owned(),value:"<operator>.indirectComputedMemberAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deprecated. Formerly in C++: `a->*b` and a[b] in Java".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:121
let constant_73=Constant{name:"indirection".to_owned(),value:"<operator>.indirection".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Accesses through a pointer (load/store), i.e. dereferences it. Typically the star-operator in C/C++. Pairs of addressOf and indirection operators cancel each other. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:122
let constant_74=Constant{name:"delete".to_owned(),value:"<operator>.delete".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deletes a property from a namespace. E.g. `a=3; delete a; a == undefined;".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:123
let constant_75=Constant{name:"conditional".to_owned(),value:"<operator>.conditional".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("E.g. `a ? consequent : alternate`. In future probably also used for if statements".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:124
let constant_76=Constant{name:"elvis".to_owned(),value:"<operator>.elvis".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("E.g. `x = f() ?: g()`. A binary operator that returns its first operand if that operand evaluates to a true value, and otherwise evaluates and returns its second operand.".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:125
let constant_77=Constant{name:"cast".to_owned(),value:"<operator>.cast".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Type casts of any sort".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:126
let constant_78=Constant{name:"compare".to_owned(),value:"<operator>.compare".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Comparison between two arguments with the results: 0 == equal, negative == left < right, positive == left > right".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:127
let constant_79=Constant{name:"addressOf".to_owned(),value:"<operator>.addressOf".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns the address of a given object. Pairs of addressOf and indirection operators cancel each other. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:128
let constant_80=Constant{name:"sizeOf".to_owned(),value:"<operator>.sizeOf".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns the size of a given object".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:129
let constant_81=Constant{name:"fieldAccess".to_owned(),value:"<operator>.fieldAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns or sets a field/member of a struct/class. Typically, the dot-operator. In terms of CPG, the first argument is the struct/class and the second argument is either a FIELD_IDENTIFIER node, a LITERAL node or an arbitrary expression. If it is a FIELD_IDENTIFIER, then the string contained in the CANONICAL_NAME field of this FIELD_IDENTIFIER node selects which field/member is accessed. If it is a LITERAL node, then its CODE selects which field/member is selected. In any other case the access is considered unpredictable by the backend, and we overtaint. indexAccess and fieldAccess live in the same space, such that e.g. obj.field and obj[\"field\"] refer to the same target (as is correct in e.g. javascript). Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:130
let constant_82=Constant{name:"indirectFieldAccess".to_owned(),value:"<operator>.indirectFieldAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Accesses (loads/stores) to a field/member through a pointer to a struct/class. Typically, C/C++ arrow-operator. The field selection works the same way as for fieldAccess. For the sake of data-flow tracking, this is the same as first dereferencing the pointer and then accessing the field. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:131
let constant_83=Constant{name:"indexAccess".to_owned(),value:"<operator>.indexAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Accesses a container (e.g. array or associative array / dict / map) at an index. The index selection works the same way as for fieldAccess. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:132
let constant_84=Constant{name:"indirectIndexAccess".to_owned(),value:"<operator>.indirectIndexAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Accesses an element of an Array through a pointer. The index selection works similar as for fieldAccess: However, the index must be an integer. If the second argument is a FIELD_ACCESS resp. LITERAL then its CANONICAL_NAME resp. CODE field is parsed as an integer; on parsing failure, the access is considered unpredictable and we overtaint. This is equivalent to a pointerShift followed by an indirection. This operator is currently only used by C-style languages. This behavior allows the backend to infer that ptr[0] and *ptr refer to the same object. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:133
let constant_85=Constant{name:"pointerShift".to_owned(),value:"<operator>.pointerShift".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Shifts a pointer. In terms of CPG, the first argument is the pointer and the second argument is the index. The index selection works the same way as for indirectIndexAccess. This operator is currently only used directly by the LLVM language, but it is also used internally for C. For example, pointerShift(ptr, 7) is equivalent to &(ptr[7]). Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:134
let constant_86=Constant{name:"getElementPtr".to_owned(),value:"<operator>.getElementPtr".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Derives a pointer to a field of a struct from a pointer to the entire struct. This corresponds to the C idiom &(ptr->field), which does not access memory. The field selection works the same way as for fieldAccess. This operator is currently only used directly by the LLVM language. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:135
let constant_87=Constant{name:"formatString".to_owned(),value:"<operator>.formatString".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Defines a string literal which can replace containing expressions with their values.".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:136
let constant_88=Constant{name:"formattedValue".to_owned(),value:"<operator>.formattedValue".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Replaces an expression with its string value.".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:137
let constant_89=Constant{name:"range".to_owned(),value:"<operator>.range".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Defines a range of values, e.g. `for (i in 1..10) print(i)`.".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:138
let constant_90=Constant{name:"in".to_owned(),value:"<operator>.in".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Checks the existence of a variable in a range or collection, e.g. `for (i in 1..10) print(i)`.".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:139
let constant_91=Constant{name:"notIn".to_owned(),value:"<operator>.notIn".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Checks the non-existence of a variable in a range or collection, e.g. `print(5 not in [1, 2, 3, 4])`".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:140
let constant_92=Constant{name:"is".to_owned(),value:"<operator>.is".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Checks whether a variable is of a given type".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:141
let constant_93=Constant{name:"isNot".to_owned(),value:"<operator>.isNot".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Checks whether a variable is not of a given type".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:142
let constant_94=Constant{name:"notNullAssert".to_owned(),value:"<operator>.notNullAssert".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Converts any value to a not-null type".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:143
let constant_95=Constant{name:"lengthOf".to_owned(),value:"<operator>.lengthOf".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns the length of the given collection e.g. (new int[]{ 1, 2, 3 }).length in Java or len([1, 2, 3]) in Python".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:144
let constant_96=Constant{name:"safeNavigation".to_owned(),value:"<operator>.safeNavigation".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns null if the first operator is null, otherwise performs a dereferencing operation".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:145
let constant_97=Constant{name:"tryCatch".to_owned(),value:"<operator>.tryCatch".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns either the last expression in the `try`-block, or the last expression in one of the `catch`/`except`-blocks. The contents of the `finally`-block are ignored.".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:146
let constant_98=Constant{name:"alloc".to_owned(),value:"<operator>.alloc".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Allocates memory for an object of a specific type".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:147
let constant_99=Constant{name:"importCall".to_owned(),value:"<operator>.import".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Imports a package".to_owned()),schema_info:Some(info_3.clone())};
// CPG_CONSTRUCTION_OP:148
builder.add_constants("Operators".to_owned(),[constant_26.clone(),constant_27.clone(),constant_28.clone(),constant_29.clone(),constant_30.clone(),constant_31.clone(),constant_32.clone(),constant_33.clone(),constant_34.clone(),constant_35.clone(),constant_36.clone(),constant_37.clone(),constant_38.clone(),constant_39.clone(),constant_40.clone(),constant_41.clone(),constant_42.clone(),constant_43.clone(),constant_44.clone(),constant_45.clone(),constant_46.clone(),constant_47.clone(),constant_48.clone(),constant_49.clone(),constant_50.clone(),constant_51.clone(),constant_52.clone(),constant_53.clone(),constant_54.clone(),constant_55.clone(),constant_56.clone(),constant_57.clone(),constant_58.clone(),constant_59.clone(),constant_60.clone(),constant_61.clone(),constant_62.clone(),constant_63.clone(),constant_64.clone(),constant_65.clone(),constant_66.clone(),constant_67.clone(),constant_68.clone(),constant_69.clone(),constant_70.clone(),constant_71.clone(),constant_72.clone(),constant_73.clone(),constant_74.clone(),constant_75.clone(),constant_76.clone(),constant_77.clone(),constant_78.clone(),constant_79.clone(),constant_80.clone(),constant_81.clone(),constant_82.clone(),constant_83.clone(),constant_84.clone(),constant_85.clone(),constant_86.clone(),constant_87.clone(),constant_88.clone(),constant_89.clone(),constant_90.clone(),constant_91.clone(),constant_92.clone(),constant_93.clone(),constant_94.clone(),constant_95.clone(),constant_96.clone(),constant_97.clone(),constant_98.clone(),constant_99.clone(),]);
// CPG_CONSTRUCTION_OP:149
builder.add_node_type("CONFIG_FILE".to_owned(),"This node type represent a configuration file, where `NAME` is the name\nof the file and `content` is its content. The exact representation of the\nname is left undefined and can be chosen as required by consumers of\nthe corresponding configuration files.",Some(info_4.clone()));
// CPG_CONSTRUCTION_OP:150
builder.node_mut("CONFIG_FILE")?.proto_id=Some(50);
// CPG_CONSTRUCTION_OP:151
builder.node_mut("CONFIG_FILE")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:152
builder.node_mut("CONFIG_FILE")?.add_property("CONTENT".to_owned());
// CPG_CONSTRUCTION_OP:153
builder.add_property("OVERLAYS".to_owned(),ValueType::String,"The field contains the names of the overlays applied to this CPG, in order of their\napplication. Names are free-form strings, that is, this specification does not\ndictate them but rather requires tool producers and consumers to communicate them\nbetween each other.",Some(info_5.clone()));
// CPG_CONSTRUCTION_OP:154
builder.property_mut("OVERLAYS")?.as_list();
// CPG_CONSTRUCTION_OP:155
builder.property_mut("OVERLAYS")?.proto_id=Some(118);
// CPG_CONSTRUCTION_OP:156
builder.add_property("LANGUAGE".to_owned(),ValueType::String,"This field indicates which CPG language frontend generated the CPG.\nFrontend developers may freely choose a value that describes their frontend\nso long as it is not used by an existing frontend. Reserved values are to date:\nC, LLVM, GHIDRA, PHP.",Some(info_5.clone()));
// CPG_CONSTRUCTION_OP:157
builder.property_mut("LANGUAGE")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:158
builder.property_mut("LANGUAGE")?.proto_id=Some(19);
// CPG_CONSTRUCTION_OP:159
builder.add_property("ROOT".to_owned(),ValueType::String,"The path to the root directory of the source/binary this CPG is generated from.",Some(info_5.clone()));
// CPG_CONSTRUCTION_OP:160
builder.property_mut("ROOT")?.proto_id=Some(1199);
// CPG_CONSTRUCTION_OP:161
builder.property_mut("ROOT")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:162
builder.add_node_type("META_DATA".to_owned(),"This node contains the CPG meta data. Exactly one node of this type\nMUST exist per CPG. The `HASH` property MAY contain a hash value calculated\nover the source files this CPG was generated from. The `VERSION` MUST be\nset to the version of the specification (\"1.1\"). The language field indicates\nwhich language frontend was used to generate the CPG and the list property\n`OVERLAYS` specifies which overlays have been applied to the CPG.",Some(info_5.clone()));
// CPG_CONSTRUCTION_OP:163
builder.node_mut("META_DATA")?.proto_id=Some(39);
// CPG_CONSTRUCTION_OP:164
builder.node_mut("META_DATA")?.add_property("LANGUAGE".to_owned());
// CPG_CONSTRUCTION_OP:165
builder.node_mut("META_DATA")?.add_property("VERSION".to_owned());
// CPG_CONSTRUCTION_OP:166
builder.node_mut("META_DATA")?.add_property("OVERLAYS".to_owned());
// CPG_CONSTRUCTION_OP:167
builder.node_mut("META_DATA")?.add_property("HASH".to_owned());
// CPG_CONSTRUCTION_OP:168
builder.node_mut("META_DATA")?.add_property("ROOT".to_owned());
// CPG_CONSTRUCTION_OP:169
let mut constant_105=Constant{name:"JAVA".to_owned(),value:"JAVA".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:170
constant_105.proto_id=Some(1);
// CPG_CONSTRUCTION_OP:171
let mut constant_106=Constant{name:"JAVASCRIPT".to_owned(),value:"JAVASCRIPT".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:172
constant_106.proto_id=Some(2);
// CPG_CONSTRUCTION_OP:173
let mut constant_107=Constant{name:"GOLANG".to_owned(),value:"GOLANG".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:174
constant_107.proto_id=Some(3);
// CPG_CONSTRUCTION_OP:175
let mut constant_108=Constant{name:"CSHARP".to_owned(),value:"CSHARP".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:176
constant_108.proto_id=Some(4);
// CPG_CONSTRUCTION_OP:177
let mut constant_109=Constant{name:"ABAP".to_owned(),value:"ABAP".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:178
constant_109.proto_id=Some(20);
// CPG_CONSTRUCTION_OP:179
let mut constant_110=Constant{name:"C".to_owned(),value:"C".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:180
constant_110.proto_id=Some(5);
// CPG_CONSTRUCTION_OP:181
let mut constant_111=Constant{name:"PYTHON".to_owned(),value:"PYTHON".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:182
constant_111.proto_id=Some(6);
// CPG_CONSTRUCTION_OP:183
let mut constant_112=Constant{name:"LLVM".to_owned(),value:"LLVM".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:184
constant_112.proto_id=Some(7);
// CPG_CONSTRUCTION_OP:185
let mut constant_113=Constant{name:"PHP".to_owned(),value:"PHP".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:186
constant_113.proto_id=Some(8);
// CPG_CONSTRUCTION_OP:187
let mut constant_114=Constant{name:"FUZZY_TEST_LANG".to_owned(),value:"FUZZY_TEST_LANG".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:188
constant_114.proto_id=Some(9);
// CPG_CONSTRUCTION_OP:189
let mut constant_115=Constant{name:"GHIDRA".to_owned(),value:"GHIDRA".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("generic reverse engineering framework".to_owned()),schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:190
constant_115.proto_id=Some(10);
// CPG_CONSTRUCTION_OP:191
let mut constant_116=Constant{name:"KOTLIN".to_owned(),value:"KOTLIN".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:192
constant_116.proto_id=Some(11);
// CPG_CONSTRUCTION_OP:193
let mut constant_117=Constant{name:"NEWC".to_owned(),value:"NEWC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Eclipse CDT based parser for C/C++".to_owned()),schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:194
constant_117.proto_id=Some(12);
// CPG_CONSTRUCTION_OP:195
let mut constant_118=Constant{name:"JAVASRC".to_owned(),value:"JAVASRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based front-end for Java".to_owned()),schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:196
constant_118.proto_id=Some(13);
// CPG_CONSTRUCTION_OP:197
let mut constant_119=Constant{name:"PYTHONSRC".to_owned(),value:"PYTHONSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based front-end for Python".to_owned()),schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:198
constant_119.proto_id=Some(14);
// CPG_CONSTRUCTION_OP:199
let mut constant_120=Constant{name:"JSSRC".to_owned(),value:"JSSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based JS frontend based on Babel".to_owned()),schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:200
constant_120.proto_id=Some(15);
// CPG_CONSTRUCTION_OP:201
let mut constant_121=Constant{name:"RUBYSRC".to_owned(),value:"RUBYSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based frontend for Ruby".to_owned()),schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:202
constant_121.proto_id=Some(17);
// CPG_CONSTRUCTION_OP:203
let mut constant_122=Constant{name:"SWIFTSRC".to_owned(),value:"SWIFTSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based frontend for Swift".to_owned()),schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:204
constant_122.proto_id=Some(18);
// CPG_CONSTRUCTION_OP:205
let mut constant_123=Constant{name:"CSHARPSRC".to_owned(),value:"CSHARPSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based frontend for C# and .NET".to_owned()),schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:206
constant_123.proto_id=Some(19);
// CPG_CONSTRUCTION_OP:207
let mut constant_124=Constant{name:"RUST".to_owned(),value:"RUST".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())};
// CPG_CONSTRUCTION_OP:208
constant_124.proto_id=Some(21);
// CPG_CONSTRUCTION_OP:209
builder.add_constants("Languages".to_owned(),[constant_105.clone(),constant_106.clone(),constant_107.clone(),constant_108.clone(),constant_109.clone(),constant_110.clone(),constant_111.clone(),constant_112.clone(),constant_113.clone(),constant_114.clone(),constant_115.clone(),constant_116.clone(),constant_117.clone(),constant_118.clone(),constant_119.clone(),constant_120.clone(),constant_121.clone(),constant_122.clone(),constant_123.clone(),constant_124.clone(),]);
// CPG_CONSTRUCTION_OP:210
builder.add_property("TYPE_FULL_NAME".to_owned(),ValueType::String,"This field contains the fully-qualified static type name of the program\nconstruct represented by a node. It is the name of an instantiated type, e.g.,\n`java.util.List<Integer>`, rather than `java.util.List[T]`. If the type\ncannot be determined, this field should be set to the empty string.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:211
builder.property_mut("TYPE_FULL_NAME")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:212
builder.property_mut("TYPE_FULL_NAME")?.proto_id=Some(51);
// CPG_CONSTRUCTION_OP:213
builder.add_property("ALIAS_TYPE_FULL_NAME".to_owned(),ValueType::String,"This property holds the fully qualified name of the type that the node is\na type alias of.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:214
builder.property_mut("ALIAS_TYPE_FULL_NAME")?.proto_id=Some(158);
// CPG_CONSTRUCTION_OP:215
builder.add_property("INHERITS_FROM_TYPE_FULL_NAME".to_owned(),ValueType::String,"The static types a TYPE_DECL inherits from. This property is matched against the\nFULL_NAME of TYPE nodes and thus it is required to have at least one TYPE node\nfor each TYPE_FULL_NAME",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:216
builder.property_mut("INHERITS_FROM_TYPE_FULL_NAME")?.as_list();
// CPG_CONSTRUCTION_OP:217
builder.property_mut("INHERITS_FROM_TYPE_FULL_NAME")?.proto_id=Some(53);
// CPG_CONSTRUCTION_OP:218
builder.add_property("TYPE_DECL_FULL_NAME".to_owned(),ValueType::String,"The static type decl of a TYPE. This property is matched against the FULL_NAME\nof TYPE_DECL nodes. It is required to have exactly one TYPE_DECL for each\ndifferent TYPE_DECL_FULL_NAME",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:219
builder.property_mut("TYPE_DECL_FULL_NAME")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:220
builder.property_mut("TYPE_DECL_FULL_NAME")?.proto_id=Some(52);
// CPG_CONSTRUCTION_OP:221
builder.add_node_type("TYPE_DECL".to_owned(),"This node represents a type declaration as for example given by a class-, struct-,\nor union declaration. In contrast to a `TYPE` node, this node does not represent a\nconcrete instantiation of a type, e.g., for the parametrized type `List[T]`, it represents\n`List[T]`, but not `List[Integer]` where `Integer` is a concrete type.\n\nThe language frontend MUST create type declarations for all types declared in the\nsource program and MAY provide type declarations for types that are not declared\nbut referenced by the source program. If a declaration is present in the source\nprogram, the field `IS_EXTERNAL` is set to `false`. Otherwise, it is set to `true`.\n\nThe `FULL_NAME` field specifies the type's fully-qualified name, including\ninformation about the namespace it is contained in if applicable, the name field\nis the type's short name. Line and column number information is specified in the\noptional fields `LINE_NUMBER`, `COLUMN_NUMBER`, `LINE_NUMBER_END`, and\n`COLUMN_NUMBER_END` and the name of the source file is specified in `FILENAME`.\n\nBase types can be specified via the `INHERITS_FROM_TYPE_FULL_NAME` list, where\neach entry contains the fully-qualified name of a base type. If the type is\nknown to be an alias of another type (as for example introduced via the C\n`typedef` statement), the name of the alias is stored in `ALIAS_TYPE_FULL_NAME`.\n\nFinally, the fully qualified name of the program constructs that the type declaration\nis immediately contained in is stored in the `AST_PARENT_FULL_NAME` field\nand its type is indicated in the `AST_PARENT_TYPE` field to be one of\n`METHOD`, `TYPE_DECL` or `NAMESPACE_BLOCK`.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:222
builder.node_mut("TYPE_DECL")?.proto_id=Some(46);
// CPG_CONSTRUCTION_OP:223
builder.node_mut("TYPE_DECL")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:224
builder.node_mut("TYPE_DECL")?.add_property("FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:225
builder.node_mut("TYPE_DECL")?.add_property("IS_EXTERNAL".to_owned());
// CPG_CONSTRUCTION_OP:226
builder.node_mut("TYPE_DECL")?.add_property("INHERITS_FROM_TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:227
builder.node_mut("TYPE_DECL")?.add_property("ALIAS_TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:228
builder.node_mut("TYPE_DECL")?.add_property("FILENAME".to_owned());
// CPG_CONSTRUCTION_OP:229
builder.node_mut("TYPE_DECL")?.add_property("AST_PARENT_TYPE".to_owned());
// CPG_CONSTRUCTION_OP:230
builder.node_mut("TYPE_DECL")?.add_property("AST_PARENT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:231
builder.node_mut("TYPE_DECL")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:232
builder.add_node_type("TYPE_PARAMETER".to_owned(),"This node represents a formal type parameter, that is, the type parameter\nas given in a type-parametrized method or type declaration. Examples for\nlanguages that support type parameters are Java (via Generics) and C++\n(via templates). Apart from the standard fields of AST nodes, the type\nparameter carries only a `NAME` field that holds the parameters name.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:233
builder.node_mut("TYPE_PARAMETER")?.proto_id=Some(47);
// CPG_CONSTRUCTION_OP:234
builder.node_mut("TYPE_PARAMETER")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:235
builder.add_node_type("TYPE_ARGUMENT".to_owned(),"An (actual) type argument as used to instantiate a parametrized type, in the\nsame way an (actual) arguments provides concrete values for a parameter\nat method call sites. As it true for arguments, the method is not expected\nto  interpret the type argument. It MUST however store its code in the\n`CODE` field.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:236
builder.node_mut("TYPE_ARGUMENT")?.proto_id=Some(48);
// CPG_CONSTRUCTION_OP:237
builder.add_node_type("MEMBER".to_owned(),"This node represents a type member of a class, struct or union, e.g., for the\n type declaration `class Foo{ int i ; }`, it represents the declaration of the\n variable `i`.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:238
builder.node_mut("MEMBER")?.proto_id=Some(9);
// CPG_CONSTRUCTION_OP:239
builder.node_mut("MEMBER")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:240
builder.node_mut("MEMBER")?.add_property("AST_PARENT_TYPE".to_owned());
// CPG_CONSTRUCTION_OP:241
builder.node_mut("MEMBER")?.add_property("AST_PARENT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:242
builder.node_mut("MEMBER")?.extend("DECLARATION".to_owned());
// CPG_CONSTRUCTION_OP:243
builder.node_mut("MEMBER")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:244
builder.add_node_type("TYPE".to_owned(),"This node represents a type instance, that is, a concrete instantiation\nof a type declaration.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:245
builder.node_mut("TYPE")?.proto_id=Some(45);
// CPG_CONSTRUCTION_OP:246
builder.node_mut("TYPE")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:247
builder.node_mut("TYPE")?.add_property("FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:248
builder.node_mut("TYPE")?.add_property("TYPE_DECL_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:249
builder.node_mut("TYPE")?.starter_name=Some("typ".to_owned());
// CPG_CONSTRUCTION_OP:250
builder.node_mut("TYPE")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:251
builder.add_edge_type("BINDS_TO".to_owned(),"This edge connects type arguments to type parameters to indicate\nthat the type argument is used to instantiate the type parameter.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:252
builder.edge_mut("BINDS_TO")?.proto_id=Some(22);
// CPG_CONSTRUCTION_OP:253
builder.add_edge_type("ALIAS_OF".to_owned(),"This edge represents an alias relation between a type declaration and a type.\nThe language frontend MUST NOT create `ALIAS_OF` edges as they are created\nautomatically based on `ALIAS_TYPE_FULL_NAME` fields when the CPG is first loaded.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:254
builder.edge_mut("ALIAS_OF")?.proto_id=Some(138);
// CPG_CONSTRUCTION_OP:255
builder.add_edge_type("INHERITS_FROM".to_owned(),"Inheritance relation between a type declaration and a type. This edge MUST NOT\n be created by the language frontend as it is automatically created from\n `INHERITS_FROM_TYPE_FULL_NAME` fields then the CPG is first loaded.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:256
builder.edge_mut("INHERITS_FROM")?.proto_id=Some(23);
// CPG_CONSTRUCTION_OP:257
builder.connect("INHERITS_FROM","TYPE_DECL","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:258
builder.connect("ALIAS_OF","TYPE_DECL","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"aliasedType".to_owned(),out_doc:"".to_owned(),in_step:"aliasTypeDecl".to_owned(),in_doc:"Direct alias type declarations.".to_owned()})?;
// CPG_CONSTRUCTION_OP:259
builder.connect("SOURCE_FILE","TYPE_DECL","FILE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"typeDecl".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:260
builder.connect("BINDS_TO","TYPE_ARGUMENT","TYPE_PARAMETER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:261
builder.connect("REF","TYPE_ARGUMENT","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:262
builder.add_property("SIGNATURE".to_owned(),ValueType::String,"The method signature encodes the types of parameters in a string.\nThe string SHOULD be human readable and suitable for differentiating methods\nwith different parameter types sufficiently to allow for resolving of\nfunction overloading. The present specification does not enforce a strict\nformat for the signature, that is, it can be chosen by the frontend\nimplementor to fit the source language.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:263
builder.property_mut("SIGNATURE")?.mandatory(Scalar::String { value:"".to_owned() })?;
// CPG_CONSTRUCTION_OP:264
builder.property_mut("SIGNATURE")?.proto_id=Some(22);
// CPG_CONSTRUCTION_OP:265
builder.add_node_type("METHOD".to_owned(),"Programming languages offer many closely-related concepts for describing blocks\nof code that can be executed with input parameters and return output parameters,\npossibly causing side effects. In the CPG specification, we refer to all of these\nconcepts (procedures, functions, methods, etc.) as methods. A single METHOD node\nmust exist for each method found in the source program.\n\nThe `FULL_NAME` field specifies the method's fully-qualified name, including\ninformation about the namespace it is contained in if applicable, the name field\nis the function's short name. The field `IS_EXTERNAL` indicates whether it was\npossible to identify a method body for the method. This is true for methods that\nare defined in the source program, and false for methods that are dynamically\nlinked to the program, that is, methods that exist in an external dependency.\n\nLine and column number information is specified in the optional fields\n`LINE_NUMBER`, `COLUMN_NUMBER`, `LINE_NUMBER_END`, and `COLUMN_NUMBER_END` and\nthe name of the source file is specified in `FILENAME`. An optional hash value\nMAY be calculated over the function contents and included in the `HASH` field.\n\nFinally, the fully qualified name of the program constructs that the method\nis immediately contained in is stored in the `AST_PARENT_FULL_NAME` field\nand its type is indicated in the `AST_PARENT_TYPE` field to be one of\n`METHOD`, `TYPE_DECL` or `NAMESPACE_BLOCK`.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:266
builder.node_mut("METHOD")?.proto_id=Some(1);
// CPG_CONSTRUCTION_OP:267
builder.node_mut("METHOD")?.add_property("FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:268
builder.node_mut("METHOD")?.add_property("IS_EXTERNAL".to_owned());
// CPG_CONSTRUCTION_OP:269
builder.node_mut("METHOD")?.add_property("SIGNATURE".to_owned());
// CPG_CONSTRUCTION_OP:270
builder.node_mut("METHOD")?.add_property("LINE_NUMBER_END".to_owned());
// CPG_CONSTRUCTION_OP:271
builder.node_mut("METHOD")?.add_property("COLUMN_NUMBER_END".to_owned());
// CPG_CONSTRUCTION_OP:272
builder.node_mut("METHOD")?.add_property("FILENAME".to_owned());
// CPG_CONSTRUCTION_OP:273
builder.node_mut("METHOD")?.add_property("HASH".to_owned());
// CPG_CONSTRUCTION_OP:274
builder.node_mut("METHOD")?.add_property("AST_PARENT_TYPE".to_owned());
// CPG_CONSTRUCTION_OP:275
builder.node_mut("METHOD")?.add_property("AST_PARENT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:276
builder.node_mut("METHOD")?.extend("DECLARATION".to_owned());
// CPG_CONSTRUCTION_OP:277
builder.node_mut("METHOD")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:278
builder.add_property("IS_VARIADIC".to_owned(),ValueType::Boolean,"Specifies whether a parameter is the variadic argument handling parameter of\na variadic method. Only one parameter of a method is allowed to have this\nproperty set to true.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:279
builder.property_mut("IS_VARIADIC")?.mandatory(Scalar::Boolean { value:false })?;
// CPG_CONSTRUCTION_OP:280
builder.property_mut("IS_VARIADIC")?.proto_id=Some(221);
// CPG_CONSTRUCTION_OP:281
builder.add_node_type("METHOD_PARAMETER_IN".to_owned(),"This node represents a formal input parameter. The field `NAME` contains its\nname, while the field `TYPE_FULL_NAME` contains the fully qualified type name.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:282
builder.node_mut("METHOD_PARAMETER_IN")?.proto_id=Some(34);
// CPG_CONSTRUCTION_OP:283
builder.node_mut("METHOD_PARAMETER_IN")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:284
builder.node_mut("METHOD_PARAMETER_IN")?.add_property("IS_VARIADIC".to_owned());
// CPG_CONSTRUCTION_OP:285
builder.node_mut("METHOD_PARAMETER_IN")?.add_property("INDEX".to_owned());
// CPG_CONSTRUCTION_OP:286
builder.node_mut("METHOD_PARAMETER_IN")?.extend("DECLARATION".to_owned());
// CPG_CONSTRUCTION_OP:287
builder.node_mut("METHOD_PARAMETER_IN")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:288
builder.add_node_type("METHOD_PARAMETER_OUT".to_owned(),"This node represents a formal output parameter. Corresponding output parameters\nfor input parameters MUST NOT be created by the frontend as they are automatically\ncreated upon first loading the CPG.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:289
builder.node_mut("METHOD_PARAMETER_OUT")?.proto_id=Some(33);
// CPG_CONSTRUCTION_OP:290
builder.node_mut("METHOD_PARAMETER_OUT")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:291
builder.node_mut("METHOD_PARAMETER_OUT")?.add_property("IS_VARIADIC".to_owned());
// CPG_CONSTRUCTION_OP:292
builder.node_mut("METHOD_PARAMETER_OUT")?.add_property("INDEX".to_owned());
// CPG_CONSTRUCTION_OP:293
builder.node_mut("METHOD_PARAMETER_OUT")?.extend("DECLARATION".to_owned());
// CPG_CONSTRUCTION_OP:294
builder.add_node_type("METHOD_RETURN".to_owned(),"This node represents an (unnamed) formal method return parameter. It carries its\nfully qualified type name in `TYPE_FULL_NAME`. The `CODE` field MAY be set freely,\ne.g., to the constant `RET`, however, subsequent layer creators MUST NOT depend\non this value.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:295
builder.node_mut("METHOD_RETURN")?.proto_id=Some(3);
// CPG_CONSTRUCTION_OP:296
builder.node_mut("METHOD_RETURN")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:297
builder.connect("SOURCE_FILE","METHOD","FILE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"method".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:298
builder.add_property("ORDER".to_owned(),ValueType::Int,"This integer indicates the position of the node among\nits siblings in the AST. The left-most child has an\norder of 0.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:299
builder.property_mut("ORDER")?.mandatory(Scalar::Int { decimal:"-1".to_owned() })?;
// CPG_CONSTRUCTION_OP:300
builder.property_mut("ORDER")?.proto_id=Some(4);
// CPG_CONSTRUCTION_OP:301
builder.add_node_base_type("AST_NODE".to_owned(),"This is the base type for all nodes of the abstract syntax tree (AST). An AST\nnode has a `CODE` and an `ORDER` field. The `CODE` field contains the\ncode (verbatim) represented by the AST node. The `ORDER` field contains the\nnodes position among its siblings, encoded as an integer where the left most\nsibling has the position `0`.\n\nAST nodes contain optional `LINE_NUMBER` and `COLUMN_NUMBER` fields. For\nsource-based frontends, these fields contain the start line number and\nstart column number of the code represented by the node.\nFor machine-code-based and bytecode-based frontends, `LINE_NUMBER` contains\nthe address at which the code starts while `COLUMN_NUMBER` is undefined.\n\nThe optional `OFFSET` and `OFFSET_END` specify the start\nand exclusive end position of the code belonging to a node within the corresponding\n`FILE` nodes `CONTENT` property.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:302
builder.node_mut("AST_NODE")?.add_property("ORDER".to_owned());
// CPG_CONSTRUCTION_OP:303
builder.node_mut("AST_NODE")?.add_property("CODE".to_owned());
// CPG_CONSTRUCTION_OP:304
builder.node_mut("AST_NODE")?.add_property("LINE_NUMBER".to_owned());
// CPG_CONSTRUCTION_OP:305
builder.node_mut("AST_NODE")?.add_property("COLUMN_NUMBER".to_owned());
// CPG_CONSTRUCTION_OP:306
builder.node_mut("AST_NODE")?.add_property("OFFSET".to_owned());
// CPG_CONSTRUCTION_OP:307
builder.node_mut("AST_NODE")?.add_property("OFFSET_END".to_owned());
// CPG_CONSTRUCTION_OP:308
builder.node_mut("METHOD")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:309
builder.node_mut("METHOD_PARAMETER_IN")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:310
builder.node_mut("METHOD_PARAMETER_OUT")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:311
builder.node_mut("TYPE_DECL")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:312
builder.node_mut("MEMBER")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:313
builder.node_mut("TYPE_PARAMETER")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:314
builder.node_mut("TYPE_ARGUMENT")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:315
builder.node_mut("FILE")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:316
builder.node_mut("NAMESPACE_BLOCK")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:317
builder.add_node_type("BLOCK".to_owned(),"This node represents a compound statement. Compound statements are used in many languages to allow\ngrouping a sequence of statements. For example, in C and Java, compound statements\nare statements enclosed by curly braces. Function/Method bodies are compound\nstatements. We do not use the term \"compound statement\" because \"statement\" would\nimply that the block does not yield a value upon evaluation, that is, that it is\nnot an expression. This is true in languages such as C and Java, but not for languages\nsuch as Scala where the value of the block is given by that of the last expression it\ncontains. In fact, the Scala grammar uses the term \"BlockExpr\" (short for\n\"block expression\") to describe what in the CPG we call \"Block\".",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:318
builder.node_mut("BLOCK")?.proto_id=Some(31);
// CPG_CONSTRUCTION_OP:319
builder.node_mut("BLOCK")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:320
builder.add_node_type("LITERAL".to_owned(),"This node represents a literal such as an integer or string constant. Literals\nare symbols included in the code in verbatim form and which are immutable.\nThe `TYPE_FULL_NAME` field stores the literal's fully-qualified type name,\ne.g., `java.lang.Integer`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:321
builder.node_mut("LITERAL")?.proto_id=Some(8);
// CPG_CONSTRUCTION_OP:322
builder.node_mut("LITERAL")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:323
builder.node_mut("LITERAL")?.primary_key=Some("CODE".to_owned());
// CPG_CONSTRUCTION_OP:324
builder.add_node_type("LOCAL".to_owned(),"This node represents a local variable. Its fully qualified type name is stored\nin the `TYPE_FULL_NAME` field and its name in the `NAME` field. The `CODE` field\ncontains the entire local variable declaration without initialization, e.g., for\n`int x = 10;`, it contains `int x`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:325
builder.node_mut("LOCAL")?.proto_id=Some(23);
// CPG_CONSTRUCTION_OP:326
builder.node_mut("LOCAL")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:327
builder.node_mut("LOCAL")?.extend("DECLARATION".to_owned());builder.node_mut("LOCAL")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:328
builder.node_mut("LOCAL")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:329
builder.add_node_type("IDENTIFIER".to_owned(),"This node represents an identifier as used when referring to a variable by name.\nIt holds the identifier's name in the `NAME` field and its fully-qualified type\nname in `TYPE_FULL_NAME`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:330
builder.node_mut("IDENTIFIER")?.proto_id=Some(27);
// CPG_CONSTRUCTION_OP:331
builder.node_mut("IDENTIFIER")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:332
builder.node_mut("IDENTIFIER")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:333
builder.node_mut("IDENTIFIER")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:334
builder.add_property("CANONICAL_NAME".to_owned(),ValueType::String,"This field holds the canonical name of a `FIELD_IDENTIFIER`. It is typically\nidentical to the CODE field, but canonicalized according to source language\nsemantics. Human readable names are preferable. `FIELD_IDENTIFIER` nodes must\nshare identical `CANONICAL_NAME` if and\nonly if they alias, e.g., in C-style unions (if the aliasing relationship is\nunknown or there are partial overlaps, then one must make a reasonable guess,\nand trade off between false negatives and false positives).",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:335
builder.property_mut("CANONICAL_NAME")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:336
builder.property_mut("CANONICAL_NAME")?.proto_id=Some(2001092);
// CPG_CONSTRUCTION_OP:337
builder.add_node_type("FIELD_IDENTIFIER".to_owned(),"This node represents the field accessed in a field access, e.g., in\n`a.b`, it represents `b`. The field name as it occurs in the code is\nstored in the `CODE` field. This may mean that the `CODE` field holds\nan expression. The `CANONICAL_NAME` field MAY contain the same value is\nthe `CODE` field but SHOULD contain the normalized name that results\nfrom evaluating `CODE` as an expression if such an evaluation is\npossible for the language frontend. The objective is to store an identifier\nin `CANONICAL_NAME` that is the same for two nodes iff they refer to the\nsame field, regardless of whether they use the same expression to reference\nit.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:338
builder.node_mut("FIELD_IDENTIFIER")?.proto_id=Some(2001081);
// CPG_CONSTRUCTION_OP:339
builder.node_mut("FIELD_IDENTIFIER")?.add_property("CANONICAL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:340
builder.add_property("MODIFIER_TYPE".to_owned(),ValueType::String,"The modifier type is a free-form string. The following are known modifier types:\n`STATIC`, `PUBLIC`, `PROTECTED`, `PRIVATE`, `ABSTRACT`, `NATIVE`, `CONSTRUCTOR`, `VIRTUAL`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:341
builder.property_mut("MODIFIER_TYPE")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:342
builder.property_mut("MODIFIER_TYPE")?.proto_id=Some(26);
// CPG_CONSTRUCTION_OP:343
let mut constant_152=Constant{name:"STATIC".to_owned(),value:"STATIC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The static modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:344
constant_152.proto_id=Some(1);
// CPG_CONSTRUCTION_OP:345
let mut constant_153=Constant{name:"PUBLIC".to_owned(),value:"PUBLIC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The public modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:346
constant_153.proto_id=Some(2);
// CPG_CONSTRUCTION_OP:347
let mut constant_154=Constant{name:"PROTECTED".to_owned(),value:"PROTECTED".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The protected modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:348
constant_154.proto_id=Some(3);
// CPG_CONSTRUCTION_OP:349
let mut constant_155=Constant{name:"PRIVATE".to_owned(),value:"PRIVATE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The private modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:350
constant_155.proto_id=Some(4);
// CPG_CONSTRUCTION_OP:351
let mut constant_156=Constant{name:"ABSTRACT".to_owned(),value:"ABSTRACT".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The abstract modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:352
constant_156.proto_id=Some(5);
// CPG_CONSTRUCTION_OP:353
let mut constant_157=Constant{name:"NATIVE".to_owned(),value:"NATIVE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The native modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:354
constant_157.proto_id=Some(6);
// CPG_CONSTRUCTION_OP:355
let mut constant_158=Constant{name:"CONSTRUCTOR".to_owned(),value:"CONSTRUCTOR".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The constructor modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:356
constant_158.proto_id=Some(7);
// CPG_CONSTRUCTION_OP:357
let mut constant_159=Constant{name:"VIRTUAL".to_owned(),value:"VIRTUAL".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The virtual modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:358
constant_159.proto_id=Some(8);
// CPG_CONSTRUCTION_OP:359
let mut constant_160=Constant{name:"INTERNAL".to_owned(),value:"INTERNAL".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The internal modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:360
constant_160.proto_id=Some(9);
// CPG_CONSTRUCTION_OP:361
let mut constant_161=Constant{name:"FINAL".to_owned(),value:"FINAL".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The final modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:362
constant_161.proto_id=Some(10);
// CPG_CONSTRUCTION_OP:363
let mut constant_162=Constant{name:"READONLY".to_owned(),value:"READONLY".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The readonly modifier".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:364
constant_162.proto_id=Some(11);
// CPG_CONSTRUCTION_OP:365
let mut constant_163=Constant{name:"MODULE".to_owned(),value:"MODULE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Indicate that a method defines a module in the sense e.g. a python module does with the creation of a module object".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:366
constant_163.proto_id=Some(12);
// CPG_CONSTRUCTION_OP:367
let mut constant_164=Constant{name:"LAMBDA".to_owned(),value:"LAMBDA".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Indicate that a method is an anonymous function, lambda, or closure".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:368
constant_164.proto_id=Some(13);
// CPG_CONSTRUCTION_OP:369
builder.add_constants("ModifierTypes".to_owned(),[constant_152.clone(),constant_153.clone(),constant_154.clone(),constant_155.clone(),constant_156.clone(),constant_157.clone(),constant_158.clone(),constant_159.clone(),constant_160.clone(),constant_161.clone(),constant_162.clone(),constant_163.clone(),constant_164.clone(),]);
// CPG_CONSTRUCTION_OP:370
builder.add_node_type("MODIFIER".to_owned(),"This field represents a (language-dependent) modifier such as `static`, `private`\nor `public`. Unlike most other AST nodes, it is NOT an expression, that is, it\ncannot be evaluated and cannot be passed as an argument in function calls.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:371
builder.node_mut("MODIFIER")?.proto_id=Some(300);
// CPG_CONSTRUCTION_OP:372
builder.node_mut("MODIFIER")?.add_property("MODIFIER_TYPE".to_owned());
// CPG_CONSTRUCTION_OP:373
builder.node_mut("MODIFIER")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:374
builder.add_node_type("JUMP_TARGET".to_owned(),"A jump target is any location in the code that has been specifically marked\nas the target of a jump, e.g., via a label. The `NAME` field holds the name of\nthe label while the `PARSER_TYPE_NAME` field holds the name of language construct\nthat this jump target is created from, e.g., \"Label\".",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:375
builder.node_mut("JUMP_TARGET")?.proto_id=Some(340);
// CPG_CONSTRUCTION_OP:376
builder.node_mut("JUMP_TARGET")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:377
builder.node_mut("JUMP_TARGET")?.add_property("PARSER_TYPE_NAME".to_owned());
// CPG_CONSTRUCTION_OP:378
builder.node_mut("JUMP_TARGET")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:379
builder.add_node_type("JUMP_LABEL".to_owned(),"A jump label specifies the label and thus the JUMP_TARGET of control structures\nBREAK and CONTINUE. The `NAME` field holds the name of the label while the\n`PARSER_TYPE_NAME` field holds the name of language construct that this jump\nlabel is created from, e.g., \"Label\".",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:380
builder.node_mut("JUMP_LABEL")?.proto_id=Some(341);
// CPG_CONSTRUCTION_OP:381
builder.node_mut("JUMP_LABEL")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:382
builder.node_mut("JUMP_LABEL")?.add_property("PARSER_TYPE_NAME".to_owned());
// CPG_CONSTRUCTION_OP:383
builder.node_mut("JUMP_LABEL")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:384
builder.add_node_type("METHOD_REF".to_owned(),"This node represents a reference to a method/function/procedure as it\nappears when a method is passed as an argument in a call. The `METHOD_FULL_NAME`\nfield holds the fully-qualified name of the referenced method and the\n`TYPE_FULL_NAME` holds its fully-qualified type name.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:385
builder.node_mut("METHOD_REF")?.proto_id=Some(333);
// CPG_CONSTRUCTION_OP:386
builder.node_mut("METHOD_REF")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:387
builder.add_node_type("TYPE_REF".to_owned(),"Reference to a type/class",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:388
builder.node_mut("TYPE_REF")?.proto_id=Some(335);
// CPG_CONSTRUCTION_OP:389
builder.node_mut("TYPE_REF")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:390
builder.add_node_type("RETURN".to_owned(),"This node represents a return instruction, e.g., `return x`. Note that it does\nNOT represent a formal return parameter as formal return parameters are\nrepresented via `METHOD_RETURN` nodes.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:391
builder.node_mut("RETURN")?.proto_id=Some(30);
// CPG_CONSTRUCTION_OP:392
builder.node_mut("RETURN")?.starter_name=Some("ret".to_owned());
// CPG_CONSTRUCTION_OP:393
builder.node_mut("RETURN")?.primary_key=Some("CODE".to_owned());
// CPG_CONSTRUCTION_OP:394
builder.add_property("CONTROL_STRUCTURE_TYPE".to_owned(),ValueType::String,"The `CONTROL_STRUCTURE_TYPE` field indicates which kind of control structure\na `CONTROL_STRUCTURE` node represents. The available types are the following:\n BREAK, CONTINUE, DO, WHILE, FOR, GOTO, IF, ELSE, TRY, THROW and SWITCH.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:395
builder.property_mut("CONTROL_STRUCTURE_TYPE")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:396
builder.property_mut("CONTROL_STRUCTURE_TYPE")?.proto_id=Some(27);
// CPG_CONSTRUCTION_OP:397
let mut constant_172=Constant{name:"BREAK".to_owned(),value:"BREAK".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a break statement. Labeled breaks are expected to have a JUMP_LABEL\nnode AST child with ORDER 1".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:398
constant_172.proto_id=Some(1);
// CPG_CONSTRUCTION_OP:399
let mut constant_173=Constant{name:"CONTINUE".to_owned(),value:"CONTINUE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a continue statement. Labeled continues are expected to have a JUMP_LABEL\nnode AST child with ORDER 1".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:400
constant_173.proto_id=Some(2);
// CPG_CONSTRUCTION_OP:401
let mut constant_174=Constant{name:"WHILE".to_owned(),value:"WHILE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a while statement".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:402
constant_174.proto_id=Some(3);
// CPG_CONSTRUCTION_OP:403
let mut constant_175=Constant{name:"DO".to_owned(),value:"DO".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a do statement".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:404
constant_175.proto_id=Some(4);
// CPG_CONSTRUCTION_OP:405
let mut constant_176=Constant{name:"FOR".to_owned(),value:"FOR".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a for statement".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:406
constant_176.proto_id=Some(5);
// CPG_CONSTRUCTION_OP:407
let mut constant_177=Constant{name:"GOTO".to_owned(),value:"GOTO".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a goto statement".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:408
constant_177.proto_id=Some(6);
// CPG_CONSTRUCTION_OP:409
let mut constant_178=Constant{name:"IF".to_owned(),value:"IF".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents an if statement".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:410
constant_178.proto_id=Some(7);
// CPG_CONSTRUCTION_OP:411
let mut constant_179=Constant{name:"ELSE".to_owned(),value:"ELSE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents an else statement".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:412
constant_179.proto_id=Some(8);
// CPG_CONSTRUCTION_OP:413
let mut constant_180=Constant{name:"SWITCH".to_owned(),value:"SWITCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a switch statement".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:414
constant_180.proto_id=Some(9);
// CPG_CONSTRUCTION_OP:415
let mut constant_181=Constant{name:"TRY".to_owned(),value:"TRY".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a try statement".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:416
constant_181.proto_id=Some(10);
// CPG_CONSTRUCTION_OP:417
let mut constant_182=Constant{name:"THROW".to_owned(),value:"THROW".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a throw statement".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:418
constant_182.proto_id=Some(11);
// CPG_CONSTRUCTION_OP:419
let mut constant_183=Constant{name:"MATCH".to_owned(),value:"MATCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a match expression".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:420
constant_183.proto_id=Some(12);
// CPG_CONSTRUCTION_OP:421
let mut constant_184=Constant{name:"YIELD".to_owned(),value:"YIELD".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a yield expression".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:422
constant_184.proto_id=Some(13);
// CPG_CONSTRUCTION_OP:423
let mut constant_185=Constant{name:"CATCH".to_owned(),value:"CATCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a catch clause".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:424
constant_185.proto_id=Some(14);
// CPG_CONSTRUCTION_OP:425
let mut constant_186=Constant{name:"FINALLY".to_owned(),value:"FINALLY".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a finally clause".to_owned()),schema_info:Some(info_8.clone())};
// CPG_CONSTRUCTION_OP:426
constant_186.proto_id=Some(15);
// CPG_CONSTRUCTION_OP:427
builder.add_constants("ControlStructureTypes".to_owned(),[constant_172.clone(),constant_173.clone(),constant_174.clone(),constant_175.clone(),constant_176.clone(),constant_177.clone(),constant_178.clone(),constant_179.clone(),constant_180.clone(),constant_181.clone(),constant_182.clone(),constant_183.clone(),constant_184.clone(),constant_185.clone(),constant_186.clone(),]);
// CPG_CONSTRUCTION_OP:428
builder.add_node_type("CONTROL_STRUCTURE".to_owned(),"This node represents a control structure as introduced by control structure\nstatements as well as conditional and unconditional jumps. Its type is stored in the\n`CONTROL_STRUCTURE_TYPE` field to be one of several pre-defined types. These types\n are used in the construction of the control flow layer, making it possible to\n generate the control flow layer from the abstract syntax tree layer automatically.\n\nIn addition to the `CONTROL_STRUCTURE_TYPE` field, the `PARSER_TYPE_NAME` field\nMAY be used by frontends to store the name of the control structure as emitted by\nthe parser or disassembler, however, the value of this field is not relevant\nfor construction of the control flow layer.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:429
builder.node_mut("CONTROL_STRUCTURE")?.proto_id=Some(339);
// CPG_CONSTRUCTION_OP:430
builder.node_mut("CONTROL_STRUCTURE")?.add_property("PARSER_TYPE_NAME".to_owned());
// CPG_CONSTRUCTION_OP:431
builder.node_mut("CONTROL_STRUCTURE")?.add_property("CONTROL_STRUCTURE_TYPE".to_owned());
// CPG_CONSTRUCTION_OP:432
builder.add_node_type("UNKNOWN".to_owned(),"Any AST node that the frontend would like to include in the AST but for\nwhich no suitable AST node is specified in the CPG specification may be\nincluded using a node of type `UNKNOWN`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:433
builder.node_mut("UNKNOWN")?.proto_id=Some(44);
// CPG_CONSTRUCTION_OP:434
builder.node_mut("UNKNOWN")?.add_property("PARSER_TYPE_NAME".to_owned());
// CPG_CONSTRUCTION_OP:435
builder.node_mut("UNKNOWN")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:436
builder.add_edge_type("AST".to_owned(),"This edge connects a parent node to its child in the syntax tree.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:437
builder.edge_mut("AST")?.proto_id=Some(3);
// CPG_CONSTRUCTION_OP:438
builder.add_edge_type("CONDITION".to_owned(),"The edge connects control structure nodes to the expressions that holds their conditions.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:439
builder.edge_mut("CONDITION")?.proto_id=Some(56);
// CPG_CONSTRUCTION_OP:440
builder.add_edge_type("TRUE_BODY".to_owned(),"The edge connects control structure nodes to their true branch/body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:441
builder.edge_mut("TRUE_BODY")?.proto_id=Some(60);
// CPG_CONSTRUCTION_OP:442
builder.add_edge_type("FALSE_BODY".to_owned(),"The edge connects control structure nodes to their false branch/body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:443
builder.edge_mut("FALSE_BODY")?.proto_id=Some(61);
// CPG_CONSTRUCTION_OP:444
builder.add_edge_type("DO_BODY".to_owned(),"The edge connects do-while control structure nodes to their body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:445
builder.edge_mut("DO_BODY")?.proto_id=Some(62);
// CPG_CONSTRUCTION_OP:446
builder.add_edge_type("FOR_INIT".to_owned(),"The edge connects for-loop control structure nodes to their initialization expression(s).",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:447
builder.edge_mut("FOR_INIT")?.proto_id=Some(63);
// CPG_CONSTRUCTION_OP:448
builder.add_edge_type("FOR_UPDATE".to_owned(),"The edge connects for-loop control structure nodes to their update/step expression(s).",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:449
builder.edge_mut("FOR_UPDATE")?.proto_id=Some(64);
// CPG_CONSTRUCTION_OP:450
builder.add_edge_type("FOR_BODY".to_owned(),"The edge connects for-loop control structure nodes to their body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:451
builder.edge_mut("FOR_BODY")?.proto_id=Some(65);
// CPG_CONSTRUCTION_OP:452
builder.add_edge_type("TRY_BODY".to_owned(),"The edge connects try control structure nodes to their try body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:453
builder.edge_mut("TRY_BODY")?.proto_id=Some(66);
// CPG_CONSTRUCTION_OP:454
builder.add_edge_type("CATCH_BODY".to_owned(),"The edge connects try control structure nodes to catch/handler bodies.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:455
builder.edge_mut("CATCH_BODY")?.proto_id=Some(67);
// CPG_CONSTRUCTION_OP:456
builder.add_edge_type("FINALLY_BODY".to_owned(),"The edge connects try control structure nodes to their finally body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:457
builder.edge_mut("FINALLY_BODY")?.proto_id=Some(68);
// CPG_CONSTRUCTION_OP:458
builder.add_edge_type("JUMP_ARGUMENT".to_owned(),"The edge connects jump-like control structures to the node encoding their jump target.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:459
builder.edge_mut("JUMP_ARGUMENT")?.proto_id=Some(69);
// CPG_CONSTRUCTION_OP:460
builder.connect("AST","FILE","NAMESPACE_BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:461
builder.connect("AST","MEMBER","MODIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:462
builder.connect("AST","TYPE","TYPE_ARGUMENT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:463
builder.connect("AST","TYPE_DECL","TYPE_PARAMETER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:464
builder.connect("AST","TYPE_DECL","MEMBER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"typeDecl".to_owned(),in_doc:"The type declaration this member is defined in".to_owned()})?;
// CPG_CONSTRUCTION_OP:465
builder.connect("AST","TYPE_DECL","MODIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:466
builder.connect("AST","METHOD","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::One, out_step:"methodReturn".to_owned(),out_doc:"Formal return parameters".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:467
builder.connect("AST","METHOD","METHOD_PARAMETER_IN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"parameter".to_owned(),out_doc:"Parameters of the method".to_owned(),in_step:"method".to_owned(),in_doc:"Traverse to method associated with this formal parameter".to_owned()})?;
// CPG_CONSTRUCTION_OP:468
builder.connect("AST","METHOD","MODIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:469
builder.connect("AST","METHOD","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::One, out_step:"block".to_owned(),out_doc:"Root of the abstract syntax tree".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:470
builder.connect("AST","METHOD","TYPE_PARAMETER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:471
builder.connect("AST","RETURN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:472
builder.connect("AST","RETURN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:473
builder.connect("AST","RETURN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:474
builder.connect("AST","RETURN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:475
builder.connect("AST","RETURN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:476
builder.connect("AST","RETURN","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:477
builder.connect("AST","RETURN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:478
builder.connect("AST","RETURN","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:479
builder.connect("AST","RETURN","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:480
builder.connect("AST","BLOCK","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:481
builder.connect("AST","BLOCK","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:482
builder.connect("AST","BLOCK","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:483
builder.connect("AST","BLOCK","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:484
builder.connect("AST","BLOCK","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:485
builder.connect("AST","BLOCK","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:486
builder.connect("AST","BLOCK","LOCAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"local".to_owned(),out_doc:"Traverse to locals of this block.".to_owned(),in_step:"definingBlock".to_owned(),in_doc:"The block in which local is declared.".to_owned()})?;
// CPG_CONSTRUCTION_OP:487
builder.connect("AST","BLOCK","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:488
builder.connect("AST","BLOCK","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:489
builder.connect("AST","BLOCK","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:490
builder.connect("AST","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:491
builder.connect("AST","CONTROL_STRUCTURE","MODIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:492
builder.connect("AST","CONTROL_STRUCTURE","LOCAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:493
builder.connect("AST","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:494
builder.connect("AST","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:495
builder.connect("AST","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:496
builder.connect("AST","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:497
builder.connect("AST","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:498
builder.connect("AST","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:499
builder.connect("AST","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:500
builder.connect("AST","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:501
builder.connect("AST","CONTROL_STRUCTURE","JUMP_LABEL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:502
builder.connect("AST","UNKNOWN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:503
builder.connect("AST","UNKNOWN","MODIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:504
builder.connect("AST","UNKNOWN","LOCAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:505
builder.connect("AST","UNKNOWN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:506
builder.connect("AST","UNKNOWN","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:507
builder.connect("AST","UNKNOWN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:508
builder.connect("AST","UNKNOWN","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:509
builder.connect("AST","UNKNOWN","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:510
builder.connect("AST","UNKNOWN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:511
builder.connect("AST","UNKNOWN","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:512
builder.connect("AST","UNKNOWN","MEMBER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:513
builder.connect("AST","METHOD_PARAMETER_IN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:514
builder.connect("AST","NAMESPACE_BLOCK","TYPE_DECL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"namespaceBlock".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:515
builder.connect("AST","NAMESPACE_BLOCK","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:516
builder.node_mut("NAMESPACE")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:517
builder.connect("CONDITION","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:518
builder.connect("CONDITION","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:519
builder.connect("CONDITION","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:520
builder.connect("CONDITION","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:521
builder.connect("CONDITION","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:522
builder.connect("CONDITION","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:523
builder.connect("CONDITION","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:524
builder.connect("CONDITION","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:525
builder.connect("CONDITION","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:526
builder.connect("CONDITION","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:527
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:528
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:529
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:530
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:531
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:532
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:533
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:534
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:535
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:536
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:537
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:538
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:539
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:540
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:541
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:542
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:543
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:544
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:545
builder.connect("DO_BODY","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:546
builder.connect("DO_BODY","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:547
builder.connect("DO_BODY","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:548
builder.connect("DO_BODY","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:549
builder.connect("DO_BODY","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:550
builder.connect("DO_BODY","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:551
builder.connect("DO_BODY","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:552
builder.connect("DO_BODY","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:553
builder.connect("DO_BODY","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:554
builder.connect("FOR_INIT","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:555
builder.connect("FOR_INIT","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:556
builder.connect("FOR_INIT","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:557
builder.connect("FOR_INIT","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:558
builder.connect("FOR_INIT","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:559
builder.connect("FOR_INIT","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:560
builder.connect("FOR_INIT","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:561
builder.connect("FOR_INIT","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:562
builder.connect("FOR_INIT","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:563
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:564
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:565
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:566
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:567
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:568
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:569
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:570
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:571
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:572
builder.connect("FOR_BODY","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:573
builder.connect("FOR_BODY","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:574
builder.connect("FOR_BODY","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:575
builder.connect("FOR_BODY","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:576
builder.connect("FOR_BODY","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:577
builder.connect("FOR_BODY","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:578
builder.connect("FOR_BODY","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:579
builder.connect("FOR_BODY","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:580
builder.connect("FOR_BODY","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:581
builder.connect("TRY_BODY","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:582
builder.connect("TRY_BODY","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:583
builder.connect("TRY_BODY","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:584
builder.connect("TRY_BODY","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:585
builder.connect("TRY_BODY","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:586
builder.connect("TRY_BODY","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:587
builder.connect("TRY_BODY","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:588
builder.connect("TRY_BODY","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:589
builder.connect("TRY_BODY","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:590
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:591
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:592
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:593
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:594
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:595
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:596
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:597
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:598
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:599
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:600
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:601
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:602
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:603
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:604
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:605
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:606
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:607
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:608
builder.connect("JUMP_ARGUMENT","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:609
builder.connect("JUMP_ARGUMENT","CONTROL_STRUCTURE","JUMP_LABEL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:610
builder.connect("AST","TYPE_DECL","TYPE_DECL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:611
builder.connect("AST","TYPE_DECL","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:612
builder.connect("AST","METHOD","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"method".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:613
builder.connect("AST","METHOD","TYPE_DECL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:614
builder.connect("AST","METHOD","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:615
builder.add_node_base_type("CALL_REPR".to_owned(),"This is the base class of `CALL` that language implementers may safely ignore.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:616
builder.node_mut("CALL_REPR")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:617
builder.node_mut("CALL_REPR")?.add_property("SIGNATURE".to_owned());
// CPG_CONSTRUCTION_OP:618
builder.add_node_type("CALL".to_owned(),"A (function/method/procedure) call. The `METHOD_FULL_NAME` property is the name of the\ninvoked method (the callee) while the `TYPE_FULL_NAME` is its return type, and\ntherefore, the return type of the call when viewing it as an expression. For\nlanguages like Javascript, it is common that we may know the (short-) name\nof the invoked method, but we do not know at compile time which method\nwill actually be invoked, e.g., because it depends on a dynamic import.\nIn this case, we leave `METHOD_FULL_NAME` blank but at least fill out `NAME`,\nwhich contains the method's (short-) name and `SIGNATURE`, which contains\nany information we may have about the types of arguments and return value.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:619
builder.node_mut("CALL")?.proto_id=Some(15);
// CPG_CONSTRUCTION_OP:620
builder.node_mut("CALL")?.extend("CALL_REPR".to_owned());
// CPG_CONSTRUCTION_OP:621
builder.node_mut("CALL")?.add_property("TYPE_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:622
builder.node_mut("CALL")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:623
builder.add_node_base_type("EXPRESSION".to_owned(),"`EXPRESSION` is the base class for all nodes that represent code pieces\nthat can be evaluated.\n\n Expression may be arguments in method calls. For method calls that do\n not involved named parameters, the `ARGUMENT_INDEX` field indicates at\n which position in the argument list the expression occurs, e.g., an\n `ARGUMENT_INDEX` of 1 indicates that the expression is the first argument\n in a method call. For calls that employ named parameters, `ARGUMENT_INDEX`\n is set to -1 and the `ARGUMENT_NAME` fields holds the name of the parameter.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:624
builder.node_mut("EXPRESSION")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:625
builder.node_mut("FIELD_IDENTIFIER")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:626
builder.node_mut("IDENTIFIER")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:627
builder.node_mut("LITERAL")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:628
builder.node_mut("BLOCK")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:629
builder.node_mut("CONTROL_STRUCTURE")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:630
builder.node_mut("METHOD_REF")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:631
builder.node_mut("TYPE_REF")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:632
builder.node_mut("RETURN")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:633
builder.node_mut("UNKNOWN")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:634
builder.connect("AST","CALL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:635
builder.connect("AST","CALL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:636
builder.connect("AST","CALL","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:637
builder.connect("AST","CALL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:638
builder.connect("AST","CALL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:639
builder.connect("AST","CALL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:640
builder.connect("AST","CALL","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:641
builder.connect("AST","CALL","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:642
builder.connect("AST","CALL","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:643
builder.connect("AST","RETURN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:644
builder.connect("AST","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:645
builder.connect("AST","UNKNOWN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:646
builder.connect("CONDITION","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:647
builder.connect("TRUE_BODY","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:648
builder.connect("FALSE_BODY","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:649
builder.connect("DO_BODY","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:650
builder.connect("FOR_INIT","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:651
builder.connect("FOR_UPDATE","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:652
builder.connect("FOR_BODY","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:653
builder.connect("TRY_BODY","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:654
builder.connect("CATCH_BODY","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:655
builder.connect("FINALLY_BODY","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:656
builder.connect("AST","BLOCK","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:657
builder.connect("REF","IDENTIFIER","LOCAL",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"referencingIdentifiers".to_owned(),in_doc:"Places (identifier) where this local is being referenced".to_owned()})?;
// CPG_CONSTRUCTION_OP:658
builder.connect("REF","IDENTIFIER","METHOD_PARAMETER_IN",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"referencingIdentifiers".to_owned(),in_doc:"Places (identifier) where this parameter is being referenced".to_owned()})?;
// CPG_CONSTRUCTION_OP:659
builder.connect("REF","NAMESPACE_BLOCK","NAMESPACE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:660
builder.add_property("ARGUMENT_INDEX".to_owned(),ValueType::Int,"AST-children of CALL nodes have an argument index, that is used to match\ncall-site arguments with callee parameters. Explicit parameters are numbered\nfrom 1 to N, while index 0 is reserved for implicit self / this parameter.\nCALLs without implicit parameter therefore have arguments starting with index 1.\nAST-children of BLOCK nodes may have an argument index as well; in this case,\nthe last argument index determines the return expression of a BLOCK expression.\nIf the `PARAMETER_NAME` field is set, then the `ARGUMENT_INDEX` field is\nignored. It is suggested to set it to -1.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:661
builder.property_mut("ARGUMENT_INDEX")?.mandatory(Scalar::Int { decimal:"-1".to_owned() })?;
// CPG_CONSTRUCTION_OP:662
builder.property_mut("ARGUMENT_INDEX")?.proto_id=Some(40);
// CPG_CONSTRUCTION_OP:663
builder.add_property("ARGUMENT_NAME".to_owned(),ValueType::String,"For calls involving named parameters, the `ARGUMENT_NAME` field holds the\nname of the parameter initialized by the expression. For all other calls,\nthis field is unset.\nNote that the `ARGUMENT_NAME` should be an exact match of the NAME of a\nMETHOD_PARAMETER_{IN,OUT}. It overrides ARGUMENT_INDEX for dataflow purposes.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:664
builder.property_mut("ARGUMENT_NAME")?.proto_id=Some(130);
// CPG_CONSTRUCTION_OP:665
builder.add_property("ARGUMENT_LABEL".to_owned(),ValueType::String,"This field is used to keep track of the argument label for languages that support them, such as Swift.\nIt is used in addition to `ARGUMENT_INDEX` and can be used to reconstruct the original call syntax more faithfully.\nFor example, in Swift, a method call may look like `foo(arg1: 42, arg2: \"hello\")` where `arg1` and `arg2`\nare argument labels. In this case, the `ARGUMENT_LABEL` field for the first argument would be set to `arg1`\nand for the second argument it would be set to `arg2`.\nContrary to the `ARGUMENT_NAME` the label should not be expected to match the name of any parameter,\nand is not needed for dataflow purposes at all.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:666
builder.property_mut("ARGUMENT_LABEL")?.proto_id=Some(129);
// CPG_CONSTRUCTION_OP:667
builder.add_property("METHOD_FULL_NAME".to_owned(),ValueType::String,"The FULL_NAME of a method. Used to link CALL and METHOD nodes. It is required\nto have exactly one METHOD node for each METHOD_FULL_NAME",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:668
builder.property_mut("METHOD_FULL_NAME")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:669
builder.property_mut("METHOD_FULL_NAME")?.proto_id=Some(54);
// CPG_CONSTRUCTION_OP:670
builder.node_mut("EXPRESSION")?.add_property("ARGUMENT_INDEX".to_owned());
// CPG_CONSTRUCTION_OP:671
builder.node_mut("EXPRESSION")?.add_property("ARGUMENT_NAME".to_owned());
// CPG_CONSTRUCTION_OP:672
builder.node_mut("EXPRESSION")?.add_property("ARGUMENT_LABEL".to_owned());
// CPG_CONSTRUCTION_OP:673
builder.add_property("EVALUATION_STRATEGY".to_owned(),ValueType::String,"For formal method input parameters, output parameters, and return parameters,\nthis field holds the evaluation strategy, which is one of the following:\n1) `BY_REFERENCE` indicates that the parameter is passed by reference, 2)\n`BY_VALUE` indicates that it is passed by value, that is, a copy is made,\n3) `BY_SHARING` the parameter is a pointer/reference and it is shared with\nthe caller/callee. While a copy of the pointer is made, a copy of the object\nthat it points to is not made.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:674
builder.property_mut("EVALUATION_STRATEGY")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:675
builder.property_mut("EVALUATION_STRATEGY")?.proto_id=Some(15);
// CPG_CONSTRUCTION_OP:676
let mut constant_209=Constant{name:"BY_REFERENCE".to_owned(),value:"BY_REFERENCE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("A parameter or return of a function is passed by reference which means an address is used behind the scenes".to_owned()),schema_info:Some(info_9.clone())};
// CPG_CONSTRUCTION_OP:677
constant_209.proto_id=Some(1);
// CPG_CONSTRUCTION_OP:678
let mut constant_210=Constant{name:"BY_SHARING".to_owned(),value:"BY_SHARING".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Only applicable to object parameter or return values. The pointer to the object is passed by value but the object itself is not copied and changes to it are thus propagated out of the method context".to_owned()),schema_info:Some(info_9.clone())};
// CPG_CONSTRUCTION_OP:679
constant_210.proto_id=Some(2);
// CPG_CONSTRUCTION_OP:680
let mut constant_211=Constant{name:"BY_VALUE".to_owned(),value:"BY_VALUE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("A parameter or return of a function passed by value which means a flat copy is used".to_owned()),schema_info:Some(info_9.clone())};
// CPG_CONSTRUCTION_OP:681
constant_211.proto_id=Some(3);
// CPG_CONSTRUCTION_OP:682
builder.add_constants("EvaluationStrategies".to_owned(),[constant_209.clone(),constant_210.clone(),constant_211.clone(),]);
// CPG_CONSTRUCTION_OP:683
builder.node_mut("METHOD_PARAMETER_IN")?.add_property("EVALUATION_STRATEGY".to_owned());
// CPG_CONSTRUCTION_OP:684
builder.node_mut("METHOD_RETURN")?.add_property("EVALUATION_STRATEGY".to_owned());
// CPG_CONSTRUCTION_OP:685
builder.node_mut("METHOD_PARAMETER_OUT")?.add_property("EVALUATION_STRATEGY".to_owned());
// CPG_CONSTRUCTION_OP:686
builder.add_property("DISPATCH_TYPE".to_owned(),ValueType::String,"This field holds the dispatch type of a call, which is either `STATIC_DISPATCH` or\n`DYNAMIC_DISPATCH`. For statically dispatched method calls, the call target is known\nat compile time while for dynamically dispatched calls, it can only be determined at\nruntime as it may depend on the type of an object (as is the case for virtual method\ncalls) or calculation of an offset.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:687
builder.property_mut("DISPATCH_TYPE")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:688
builder.property_mut("DISPATCH_TYPE")?.proto_id=Some(25);
// CPG_CONSTRUCTION_OP:689
builder.add_edge_type("CALL".to_owned(),"This edge connects call sites, i.e., nodes with the type `CALL`, to the\nmethod node that represent the method they invoke. The frontend MAY create\n`CALL` edges but is not required to do so. Instead, of the `METHOD_FULL_NAME`\nfield of the `CALL` node is set correctly, `CALL` edges are created\nautomatically as the CPG is first loaded.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:690
builder.edge_mut("CALL")?.default_accessor_name=Some("STATIC_CALL".to_owned());
// CPG_CONSTRUCTION_OP:691
builder.edge_mut("CALL")?.proto_id=Some(6);
// CPG_CONSTRUCTION_OP:692
builder.add_edge_type("ARGUMENT".to_owned(),"Argument edges connect call sites (node type `CALL`) to their arguments\n(node type `EXPRESSION`) as well as `RETURN` nodes to the expressions\nthat return.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:693
builder.edge_mut("ARGUMENT")?.proto_id=Some(156);
// CPG_CONSTRUCTION_OP:694
builder.add_edge_type("RECEIVER".to_owned(),"Similar to `ARGUMENT` edges, `RECEIVER` edges connect call sites\nto their receiver arguments. A receiver argument is the object on\nwhich a method operates, that is, it is the expression that is\nassigned to the `this` pointer as control is transferred to the method.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:695
builder.edge_mut("RECEIVER")?.proto_id=Some(55);
// CPG_CONSTRUCTION_OP:696
let mut constant_216=Constant{name:"STATIC_DISPATCH".to_owned(),value:"STATIC_DISPATCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("For statically dispatched calls the call target is known before program execution".to_owned()),schema_info:Some(info_9.clone())};
// CPG_CONSTRUCTION_OP:697
constant_216.proto_id=Some(1);
// CPG_CONSTRUCTION_OP:698
let mut constant_217=Constant{name:"DYNAMIC_DISPATCH".to_owned(),value:"DYNAMIC_DISPATCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("For dynamically dispatched calls the target is determined during runtime".to_owned()),schema_info:Some(info_9.clone())};
// CPG_CONSTRUCTION_OP:699
constant_217.proto_id=Some(2);
// CPG_CONSTRUCTION_OP:700
let mut constant_218=Constant{name:"INLINED".to_owned(),value:"INLINED".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("For macro expansions, code is inlined.".to_owned()),schema_info:Some(info_9.clone())};
// CPG_CONSTRUCTION_OP:701
constant_218.proto_id=Some(3);
// CPG_CONSTRUCTION_OP:702
builder.add_constants("DispatchTypes".to_owned(),[constant_216.clone(),constant_217.clone(),constant_218.clone(),]);
// CPG_CONSTRUCTION_OP:703
builder.add_property("STATIC_RECEIVER".to_owned(),ValueType::String,"The `STATIC_RECEIVER` field is used to keep track of the type on which a static method\nis called for static methods which may be inherited. This information can then be used to find\nthe true `METHOD_FULL_NAME` of the method being called during call linking. For example, if a\nclass `Foo` defines a static method `foo` and a class `Bar extends Foo`, then the `STATIC_RECEIVER`\nof a`Bar.foo()` call is `Bar` and the `METHOD_FULL_NAME` of the `foo` call is rewritten to\n`Foo.foo:<signature>`.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:704
builder.property_mut("STATIC_RECEIVER")?.proto_id=Some(59);
// CPG_CONSTRUCTION_OP:705
builder.node_mut("CALL")?.add_property("DISPATCH_TYPE".to_owned());
// CPG_CONSTRUCTION_OP:706
builder.node_mut("CALL")?.add_property("STATIC_RECEIVER".to_owned());
// CPG_CONSTRUCTION_OP:707
builder.connect("CALL","CALL","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:708
builder.node_mut("CALL")?.add_property("METHOD_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:709
builder.node_mut("CALL")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:710
builder.node_mut("JUMP_TARGET")?.add_property("ARGUMENT_INDEX".to_owned());
// CPG_CONSTRUCTION_OP:711
builder.node_mut("METHOD_REF")?.add_property("METHOD_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:712
builder.connect("RECEIVER","CALL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:713
builder.connect("RECEIVER","CALL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:714
builder.connect("RECEIVER","CALL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:715
builder.connect("RECEIVER","CALL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:716
builder.connect("RECEIVER","CALL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:717
builder.connect("RECEIVER","CALL","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:718
builder.connect("RECEIVER","CALL","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:719
builder.connect("RECEIVER","CALL","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:720
builder.connect("ARGUMENT","CALL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:721
builder.connect("ARGUMENT","CALL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:722
builder.connect("ARGUMENT","CALL","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:723
builder.connect("ARGUMENT","CALL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:724
builder.connect("ARGUMENT","CALL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:725
builder.connect("ARGUMENT","CALL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:726
builder.connect("ARGUMENT","CALL","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:727
builder.connect("ARGUMENT","CALL","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:728
builder.connect("ARGUMENT","CALL","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:729
builder.connect("ARGUMENT","CALL","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:730
builder.connect("ARGUMENT","RETURN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:731
builder.connect("ARGUMENT","RETURN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:732
builder.connect("ARGUMENT","RETURN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:733
builder.connect("ARGUMENT","RETURN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:734
builder.connect("ARGUMENT","RETURN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:735
builder.connect("ARGUMENT","RETURN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:736
builder.connect("ARGUMENT","RETURN","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:737
builder.connect("ARGUMENT","RETURN","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:738
builder.connect("ARGUMENT","RETURN","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:739
builder.connect("ARGUMENT","RETURN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:740
builder.add_node_base_type("CFG_NODE".to_owned(),"This is the base class for all control flow nodes. It is itself\na child class of `AST_NODE`, that is, all control flow graph nodes\nare also syntax tree nodes in the CPG specification.",Some(info_10.clone()));
// CPG_CONSTRUCTION_OP:741
builder.node_mut("CFG_NODE")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:742
builder.node_mut("METHOD")?.extend("CFG_NODE".to_owned());
// CPG_CONSTRUCTION_OP:743
builder.node_mut("METHOD_RETURN")?.extend("CFG_NODE".to_owned());
// CPG_CONSTRUCTION_OP:744
builder.node_mut("METHOD_PARAMETER_IN")?.extend("CFG_NODE".to_owned());
// CPG_CONSTRUCTION_OP:745
builder.node_mut("METHOD_PARAMETER_OUT")?.extend("CFG_NODE".to_owned());
// CPG_CONSTRUCTION_OP:746
builder.node_mut("EXPRESSION")?.extend("CFG_NODE".to_owned());
// CPG_CONSTRUCTION_OP:747
builder.node_mut("CALL_REPR")?.extend("CFG_NODE".to_owned());
// CPG_CONSTRUCTION_OP:748
builder.node_mut("JUMP_TARGET")?.extend("CFG_NODE".to_owned());
// CPG_CONSTRUCTION_OP:749
builder.add_edge_type("CFG".to_owned(),"This edge indicates control flow from the source to the destination node.",Some(info_10.clone()));
// CPG_CONSTRUCTION_OP:750
builder.edge_mut("CFG")?.proto_id=Some(19);
// CPG_CONSTRUCTION_OP:751
builder.connect("CFG","METHOD","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:752
builder.connect("CFG","METHOD","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"cfgFirst".to_owned(),out_doc:"First control flow graph node".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:753
builder.connect("CFG","FIELD_IDENTIFIER","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:754
builder.connect("CFG","FIELD_IDENTIFIER","CALL",EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:755
builder.connect("CFG","BLOCK","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:756
builder.connect("CFG","CALL","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:757
builder.connect("CFG","CONTROL_STRUCTURE","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:758
builder.connect("CFG","JUMP_TARGET","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:759
builder.connect("CFG","IDENTIFIER","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:760
builder.connect("CFG","LITERAL","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:761
builder.connect("CFG","METHOD_REF","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:762
builder.connect("CFG","TYPE_REF","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:763
builder.connect("CFG","UNKNOWN","CFG_NODE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:764
builder.connect("CFG","RETURN","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"toReturn".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:765
builder.connect("CFG","CALL","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:766
builder.connect("CFG","METHOD_REF","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:767
builder.connect("CFG","TYPE_REF","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:768
builder.add_edge_type("DOMINATE".to_owned(),"This edge indicates that the source node immediately dominates the destination node.",Some(info_11.clone()));
// CPG_CONSTRUCTION_OP:769
builder.edge_mut("DOMINATE")?.proto_id=Some(181);
// CPG_CONSTRUCTION_OP:770
builder.add_edge_type("POST_DOMINATE".to_owned(),"This edge indicates that the source node immediately post dominates the destination node.",Some(info_11.clone()));
// CPG_CONSTRUCTION_OP:771
builder.edge_mut("POST_DOMINATE")?.proto_id=Some(182);
// CPG_CONSTRUCTION_OP:772
builder.connect("DOMINATE","METHOD","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:773
builder.connect("DOMINATE","METHOD","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:774
builder.connect("DOMINATE","METHOD","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:775
builder.connect("DOMINATE","METHOD","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:776
builder.connect("DOMINATE","METHOD","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:777
builder.connect("DOMINATE","METHOD","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:778
builder.connect("DOMINATE","METHOD","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:779
builder.connect("DOMINATE","METHOD","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:780
builder.connect("DOMINATE","METHOD","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:781
builder.connect("DOMINATE","METHOD","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:782
builder.connect("POST_DOMINATE","METHOD_RETURN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:783
builder.connect("POST_DOMINATE","METHOD_RETURN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:784
builder.connect("POST_DOMINATE","METHOD_RETURN","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:785
builder.connect("POST_DOMINATE","METHOD_RETURN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:786
builder.connect("POST_DOMINATE","METHOD_RETURN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:787
builder.connect("POST_DOMINATE","METHOD_RETURN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:788
builder.connect("POST_DOMINATE","METHOD_RETURN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:789
builder.connect("POST_DOMINATE","METHOD_RETURN","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:790
builder.connect("POST_DOMINATE","METHOD_RETURN","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:791
builder.connect("POST_DOMINATE","METHOD_RETURN","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:792
builder.connect("POST_DOMINATE","METHOD_RETURN","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:793
builder.connect("POST_DOMINATE","METHOD_RETURN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:794
builder.connect("DOMINATE","LITERAL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:795
builder.connect("DOMINATE","LITERAL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:796
builder.connect("DOMINATE","LITERAL","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:797
builder.connect("DOMINATE","LITERAL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:798
builder.connect("DOMINATE","LITERAL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:799
builder.connect("DOMINATE","LITERAL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:800
builder.connect("DOMINATE","LITERAL","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:801
builder.connect("DOMINATE","LITERAL","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:802
builder.connect("DOMINATE","LITERAL","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:803
builder.connect("DOMINATE","LITERAL","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:804
builder.connect("DOMINATE","LITERAL","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:805
builder.connect("DOMINATE","LITERAL","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:806
builder.connect("POST_DOMINATE","LITERAL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:807
builder.connect("POST_DOMINATE","LITERAL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:808
builder.connect("POST_DOMINATE","LITERAL","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:809
builder.connect("POST_DOMINATE","LITERAL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:810
builder.connect("POST_DOMINATE","LITERAL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:811
builder.connect("POST_DOMINATE","LITERAL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:812
builder.connect("POST_DOMINATE","LITERAL","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:813
builder.connect("POST_DOMINATE","LITERAL","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:814
builder.connect("POST_DOMINATE","LITERAL","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:815
builder.connect("POST_DOMINATE","LITERAL","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:816
builder.connect("POST_DOMINATE","LITERAL","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:817
builder.connect("POST_DOMINATE","LITERAL","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:818
builder.connect("DOMINATE","CALL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:819
builder.connect("DOMINATE","CALL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:820
builder.connect("DOMINATE","CALL","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:821
builder.connect("DOMINATE","CALL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:822
builder.connect("DOMINATE","CALL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:823
builder.connect("DOMINATE","CALL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:824
builder.connect("DOMINATE","CALL","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:825
builder.connect("DOMINATE","CALL","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:826
builder.connect("DOMINATE","CALL","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:827
builder.connect("DOMINATE","CALL","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:828
builder.connect("DOMINATE","CALL","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:829
builder.connect("DOMINATE","CALL","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:830
builder.connect("POST_DOMINATE","CALL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:831
builder.connect("POST_DOMINATE","CALL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:832
builder.connect("POST_DOMINATE","CALL","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:833
builder.connect("POST_DOMINATE","CALL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:834
builder.connect("POST_DOMINATE","CALL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:835
builder.connect("POST_DOMINATE","CALL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:836
builder.connect("POST_DOMINATE","CALL","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:837
builder.connect("POST_DOMINATE","CALL","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:838
builder.connect("POST_DOMINATE","CALL","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:839
builder.connect("POST_DOMINATE","CALL","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:840
builder.connect("POST_DOMINATE","CALL","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:841
builder.connect("POST_DOMINATE","CALL","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:842
builder.connect("DOMINATE","IDENTIFIER","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:843
builder.connect("DOMINATE","IDENTIFIER","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:844
builder.connect("DOMINATE","IDENTIFIER","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:845
builder.connect("DOMINATE","IDENTIFIER","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:846
builder.connect("DOMINATE","IDENTIFIER","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:847
builder.connect("DOMINATE","IDENTIFIER","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:848
builder.connect("DOMINATE","IDENTIFIER","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:849
builder.connect("DOMINATE","IDENTIFIER","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:850
builder.connect("DOMINATE","IDENTIFIER","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:851
builder.connect("DOMINATE","IDENTIFIER","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:852
builder.connect("DOMINATE","IDENTIFIER","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:853
builder.connect("DOMINATE","IDENTIFIER","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:854
builder.connect("POST_DOMINATE","IDENTIFIER","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:855
builder.connect("POST_DOMINATE","IDENTIFIER","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:856
builder.connect("POST_DOMINATE","IDENTIFIER","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:857
builder.connect("POST_DOMINATE","IDENTIFIER","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:858
builder.connect("POST_DOMINATE","IDENTIFIER","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:859
builder.connect("POST_DOMINATE","IDENTIFIER","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:860
builder.connect("POST_DOMINATE","IDENTIFIER","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:861
builder.connect("POST_DOMINATE","IDENTIFIER","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:862
builder.connect("POST_DOMINATE","IDENTIFIER","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:863
builder.connect("POST_DOMINATE","IDENTIFIER","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:864
builder.connect("POST_DOMINATE","IDENTIFIER","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:865
builder.connect("POST_DOMINATE","IDENTIFIER","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:866
builder.connect("DOMINATE","FIELD_IDENTIFIER","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:867
builder.connect("DOMINATE","FIELD_IDENTIFIER","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:868
builder.connect("DOMINATE","FIELD_IDENTIFIER","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:869
builder.connect("DOMINATE","FIELD_IDENTIFIER","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:870
builder.connect("DOMINATE","FIELD_IDENTIFIER","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:871
builder.connect("DOMINATE","FIELD_IDENTIFIER","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:872
builder.connect("DOMINATE","FIELD_IDENTIFIER","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:873
builder.connect("DOMINATE","FIELD_IDENTIFIER","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:874
builder.connect("DOMINATE","FIELD_IDENTIFIER","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:875
builder.connect("DOMINATE","FIELD_IDENTIFIER","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:876
builder.connect("DOMINATE","FIELD_IDENTIFIER","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:877
builder.connect("DOMINATE","FIELD_IDENTIFIER","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:878
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:879
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:880
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:881
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:882
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:883
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:884
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:885
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:886
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:887
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:888
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:889
builder.connect("POST_DOMINATE","FIELD_IDENTIFIER","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:890
builder.connect("DOMINATE","RETURN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:891
builder.connect("DOMINATE","RETURN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:892
builder.connect("DOMINATE","RETURN","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:893
builder.connect("DOMINATE","RETURN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:894
builder.connect("DOMINATE","RETURN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:895
builder.connect("DOMINATE","RETURN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:896
builder.connect("DOMINATE","RETURN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:897
builder.connect("DOMINATE","RETURN","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:898
builder.connect("DOMINATE","RETURN","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:899
builder.connect("DOMINATE","RETURN","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:900
builder.connect("DOMINATE","RETURN","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:901
builder.connect("DOMINATE","RETURN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:902
builder.connect("POST_DOMINATE","RETURN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:903
builder.connect("POST_DOMINATE","RETURN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:904
builder.connect("POST_DOMINATE","RETURN","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:905
builder.connect("POST_DOMINATE","RETURN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:906
builder.connect("POST_DOMINATE","RETURN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:907
builder.connect("POST_DOMINATE","RETURN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:908
builder.connect("POST_DOMINATE","RETURN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:909
builder.connect("POST_DOMINATE","RETURN","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:910
builder.connect("POST_DOMINATE","RETURN","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:911
builder.connect("POST_DOMINATE","RETURN","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:912
builder.connect("POST_DOMINATE","RETURN","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:913
builder.connect("POST_DOMINATE","RETURN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:914
builder.connect("DOMINATE","BLOCK","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:915
builder.connect("DOMINATE","BLOCK","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:916
builder.connect("DOMINATE","BLOCK","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:917
builder.connect("DOMINATE","BLOCK","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:918
builder.connect("DOMINATE","BLOCK","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:919
builder.connect("DOMINATE","BLOCK","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:920
builder.connect("DOMINATE","BLOCK","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:921
builder.connect("DOMINATE","BLOCK","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:922
builder.connect("DOMINATE","BLOCK","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:923
builder.connect("DOMINATE","BLOCK","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:924
builder.connect("DOMINATE","BLOCK","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:925
builder.connect("DOMINATE","BLOCK","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:926
builder.connect("POST_DOMINATE","BLOCK","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:927
builder.connect("POST_DOMINATE","BLOCK","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:928
builder.connect("POST_DOMINATE","BLOCK","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:929
builder.connect("POST_DOMINATE","BLOCK","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:930
builder.connect("POST_DOMINATE","BLOCK","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:931
builder.connect("POST_DOMINATE","BLOCK","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:932
builder.connect("POST_DOMINATE","BLOCK","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:933
builder.connect("POST_DOMINATE","BLOCK","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:934
builder.connect("POST_DOMINATE","BLOCK","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:935
builder.connect("POST_DOMINATE","BLOCK","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:936
builder.connect("POST_DOMINATE","BLOCK","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:937
builder.connect("POST_DOMINATE","BLOCK","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:938
builder.connect("DOMINATE","UNKNOWN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:939
builder.connect("DOMINATE","UNKNOWN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:940
builder.connect("DOMINATE","UNKNOWN","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:941
builder.connect("DOMINATE","UNKNOWN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:942
builder.connect("DOMINATE","UNKNOWN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:943
builder.connect("DOMINATE","UNKNOWN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:944
builder.connect("DOMINATE","UNKNOWN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:945
builder.connect("DOMINATE","UNKNOWN","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:946
builder.connect("DOMINATE","UNKNOWN","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:947
builder.connect("DOMINATE","UNKNOWN","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:948
builder.connect("DOMINATE","UNKNOWN","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:949
builder.connect("DOMINATE","UNKNOWN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:950
builder.connect("POST_DOMINATE","UNKNOWN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:951
builder.connect("POST_DOMINATE","UNKNOWN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:952
builder.connect("POST_DOMINATE","UNKNOWN","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:953
builder.connect("POST_DOMINATE","UNKNOWN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:954
builder.connect("POST_DOMINATE","UNKNOWN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:955
builder.connect("POST_DOMINATE","UNKNOWN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:956
builder.connect("POST_DOMINATE","UNKNOWN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:957
builder.connect("POST_DOMINATE","UNKNOWN","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:958
builder.connect("POST_DOMINATE","UNKNOWN","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:959
builder.connect("POST_DOMINATE","UNKNOWN","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:960
builder.connect("POST_DOMINATE","UNKNOWN","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:961
builder.connect("POST_DOMINATE","UNKNOWN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:962
builder.connect("DOMINATE","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:963
builder.connect("DOMINATE","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:964
builder.connect("DOMINATE","CONTROL_STRUCTURE","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:965
builder.connect("DOMINATE","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:966
builder.connect("DOMINATE","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:967
builder.connect("DOMINATE","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:968
builder.connect("DOMINATE","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:969
builder.connect("DOMINATE","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:970
builder.connect("DOMINATE","CONTROL_STRUCTURE","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:971
builder.connect("DOMINATE","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:972
builder.connect("DOMINATE","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:973
builder.connect("DOMINATE","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:974
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:975
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:976
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:977
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:978
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:979
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:980
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:981
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:982
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:983
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:984
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:985
builder.connect("POST_DOMINATE","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:986
builder.connect("DOMINATE","METHOD_REF","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:987
builder.connect("DOMINATE","METHOD_REF","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:988
builder.connect("DOMINATE","METHOD_REF","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:989
builder.connect("DOMINATE","METHOD_REF","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:990
builder.connect("DOMINATE","METHOD_REF","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:991
builder.connect("DOMINATE","METHOD_REF","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:992
builder.connect("DOMINATE","METHOD_REF","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:993
builder.connect("DOMINATE","METHOD_REF","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:994
builder.connect("DOMINATE","METHOD_REF","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:995
builder.connect("DOMINATE","METHOD_REF","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:996
builder.connect("DOMINATE","METHOD_REF","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:997
builder.connect("DOMINATE","METHOD_REF","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:998
builder.connect("POST_DOMINATE","METHOD_REF","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:999
builder.connect("POST_DOMINATE","METHOD_REF","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1000
builder.connect("POST_DOMINATE","METHOD_REF","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1001
builder.connect("POST_DOMINATE","METHOD_REF","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1002
builder.connect("POST_DOMINATE","METHOD_REF","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1003
builder.connect("POST_DOMINATE","METHOD_REF","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1004
builder.connect("POST_DOMINATE","METHOD_REF","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1005
builder.connect("POST_DOMINATE","METHOD_REF","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1006
builder.connect("POST_DOMINATE","METHOD_REF","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1007
builder.connect("POST_DOMINATE","METHOD_REF","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1008
builder.connect("POST_DOMINATE","METHOD_REF","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1009
builder.connect("POST_DOMINATE","METHOD_REF","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1010
builder.connect("DOMINATE","TYPE_REF","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1011
builder.connect("DOMINATE","TYPE_REF","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1012
builder.connect("DOMINATE","TYPE_REF","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1013
builder.connect("DOMINATE","TYPE_REF","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1014
builder.connect("DOMINATE","TYPE_REF","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1015
builder.connect("DOMINATE","TYPE_REF","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1016
builder.connect("DOMINATE","TYPE_REF","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1017
builder.connect("DOMINATE","TYPE_REF","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1018
builder.connect("DOMINATE","TYPE_REF","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1019
builder.connect("DOMINATE","TYPE_REF","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1020
builder.connect("DOMINATE","TYPE_REF","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1021
builder.connect("DOMINATE","TYPE_REF","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1022
builder.connect("POST_DOMINATE","TYPE_REF","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1023
builder.connect("POST_DOMINATE","TYPE_REF","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1024
builder.connect("POST_DOMINATE","TYPE_REF","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1025
builder.connect("POST_DOMINATE","TYPE_REF","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1026
builder.connect("POST_DOMINATE","TYPE_REF","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1027
builder.connect("POST_DOMINATE","TYPE_REF","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1028
builder.connect("POST_DOMINATE","TYPE_REF","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1029
builder.connect("POST_DOMINATE","TYPE_REF","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1030
builder.connect("POST_DOMINATE","TYPE_REF","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1031
builder.connect("POST_DOMINATE","TYPE_REF","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1032
builder.connect("POST_DOMINATE","TYPE_REF","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1033
builder.connect("POST_DOMINATE","TYPE_REF","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1034
builder.connect("DOMINATE","JUMP_TARGET","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1035
builder.connect("DOMINATE","JUMP_TARGET","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1036
builder.connect("DOMINATE","JUMP_TARGET","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1037
builder.connect("DOMINATE","JUMP_TARGET","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1038
builder.connect("DOMINATE","JUMP_TARGET","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1039
builder.connect("DOMINATE","JUMP_TARGET","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1040
builder.connect("DOMINATE","JUMP_TARGET","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1041
builder.connect("DOMINATE","JUMP_TARGET","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1042
builder.connect("DOMINATE","JUMP_TARGET","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1043
builder.connect("DOMINATE","JUMP_TARGET","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1044
builder.connect("DOMINATE","JUMP_TARGET","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1045
builder.connect("POST_DOMINATE","JUMP_TARGET","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1046
builder.connect("POST_DOMINATE","JUMP_TARGET","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1047
builder.connect("POST_DOMINATE","JUMP_TARGET","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1048
builder.connect("POST_DOMINATE","JUMP_TARGET","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1049
builder.connect("POST_DOMINATE","JUMP_TARGET","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1050
builder.connect("POST_DOMINATE","JUMP_TARGET","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1051
builder.connect("POST_DOMINATE","JUMP_TARGET","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1052
builder.connect("POST_DOMINATE","JUMP_TARGET","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1053
builder.connect("POST_DOMINATE","JUMP_TARGET","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1054
builder.connect("POST_DOMINATE","JUMP_TARGET","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1055
builder.connect("POST_DOMINATE","JUMP_TARGET","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1056
builder.add_property("VARIABLE".to_owned(),ValueType::String,"This edge property represents the variable propagated by a reaching definition edge.",Some(info_12.clone()));
// CPG_CONSTRUCTION_OP:1057
builder.property_mut("VARIABLE")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:1058
builder.property_mut("VARIABLE")?.proto_id=Some(11);
// CPG_CONSTRUCTION_OP:1059
builder.add_edge_type("CDG".to_owned(),"A CDG edge expresses that the destination node is control dependent on the source node.",Some(info_12.clone()));
// CPG_CONSTRUCTION_OP:1060
builder.edge_mut("CDG")?.proto_id=Some(183);
// CPG_CONSTRUCTION_OP:1061
builder.add_edge_type("REACHING_DEF".to_owned(),"A reaching definition edge indicates that a variable produced at the source node reaches\nthe destination node without being reassigned on the way. The `VARIABLE` property indicates\nwhich variable is propagated.",Some(info_12.clone()));
// CPG_CONSTRUCTION_OP:1062
builder.edge_mut("REACHING_DEF")?.proto_id=Some(137);
// CPG_CONSTRUCTION_OP:1063
builder.edge_mut("REACHING_DEF")?.property=Some("VARIABLE".to_owned());
// CPG_CONSTRUCTION_OP:1064
builder.connect("CDG","LITERAL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1065
builder.connect("CDG","LITERAL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1066
builder.connect("CDG","LITERAL","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1067
builder.connect("CDG","LITERAL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1068
builder.connect("CDG","LITERAL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1069
builder.connect("CDG","LITERAL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1070
builder.connect("CDG","LITERAL","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1071
builder.connect("CDG","LITERAL","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1072
builder.connect("CDG","LITERAL","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1073
builder.connect("CDG","LITERAL","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1074
builder.connect("CDG","LITERAL","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1075
builder.connect("CDG","LITERAL","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1076
builder.connect("CDG","CALL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1077
builder.connect("CDG","CALL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1078
builder.connect("CDG","CALL","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1079
builder.connect("CDG","CALL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1080
builder.connect("CDG","CALL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1081
builder.connect("CDG","CALL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1082
builder.connect("CDG","CALL","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1083
builder.connect("CDG","CALL","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1084
builder.connect("CDG","CALL","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1085
builder.connect("CDG","CALL","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1086
builder.connect("CDG","CALL","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1087
builder.connect("CDG","CALL","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1088
builder.connect("CDG","IDENTIFIER","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1089
builder.connect("CDG","IDENTIFIER","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1090
builder.connect("CDG","IDENTIFIER","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1091
builder.connect("CDG","IDENTIFIER","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1092
builder.connect("CDG","IDENTIFIER","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1093
builder.connect("CDG","IDENTIFIER","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1094
builder.connect("CDG","IDENTIFIER","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1095
builder.connect("CDG","IDENTIFIER","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1096
builder.connect("CDG","IDENTIFIER","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1097
builder.connect("CDG","IDENTIFIER","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1098
builder.connect("CDG","IDENTIFIER","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1099
builder.connect("CDG","IDENTIFIER","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1100
builder.connect("CDG","FIELD_IDENTIFIER","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1101
builder.connect("CDG","FIELD_IDENTIFIER","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1102
builder.connect("CDG","FIELD_IDENTIFIER","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1103
builder.connect("CDG","FIELD_IDENTIFIER","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1104
builder.connect("CDG","FIELD_IDENTIFIER","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1105
builder.connect("CDG","FIELD_IDENTIFIER","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1106
builder.connect("CDG","FIELD_IDENTIFIER","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1107
builder.connect("CDG","FIELD_IDENTIFIER","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1108
builder.connect("CDG","FIELD_IDENTIFIER","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1109
builder.connect("CDG","FIELD_IDENTIFIER","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1110
builder.connect("CDG","FIELD_IDENTIFIER","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1111
builder.connect("CDG","FIELD_IDENTIFIER","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1112
builder.connect("CDG","BLOCK","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1113
builder.connect("CDG","BLOCK","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1114
builder.connect("CDG","BLOCK","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1115
builder.connect("CDG","BLOCK","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1116
builder.connect("CDG","BLOCK","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1117
builder.connect("CDG","BLOCK","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1118
builder.connect("CDG","BLOCK","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1119
builder.connect("CDG","BLOCK","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1120
builder.connect("CDG","BLOCK","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1121
builder.connect("CDG","BLOCK","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1122
builder.connect("CDG","BLOCK","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1123
builder.connect("CDG","BLOCK","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1124
builder.connect("CDG","UNKNOWN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1125
builder.connect("CDG","UNKNOWN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1126
builder.connect("CDG","UNKNOWN","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1127
builder.connect("CDG","UNKNOWN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1128
builder.connect("CDG","UNKNOWN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1129
builder.connect("CDG","UNKNOWN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1130
builder.connect("CDG","UNKNOWN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1131
builder.connect("CDG","UNKNOWN","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1132
builder.connect("CDG","UNKNOWN","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1133
builder.connect("CDG","UNKNOWN","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1134
builder.connect("CDG","UNKNOWN","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1135
builder.connect("CDG","UNKNOWN","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1136
builder.connect("CDG","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1137
builder.connect("CDG","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1138
builder.connect("CDG","CONTROL_STRUCTURE","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1139
builder.connect("CDG","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1140
builder.connect("CDG","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1141
builder.connect("CDG","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1142
builder.connect("CDG","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1143
builder.connect("CDG","CONTROL_STRUCTURE","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1144
builder.connect("CDG","CONTROL_STRUCTURE","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1145
builder.connect("CDG","CONTROL_STRUCTURE","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1146
builder.connect("CDG","CONTROL_STRUCTURE","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1147
builder.connect("CDG","CONTROL_STRUCTURE","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1148
builder.connect("CDG","METHOD_REF","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1149
builder.connect("CDG","METHOD_REF","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1150
builder.connect("CDG","METHOD_REF","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1151
builder.connect("CDG","METHOD_REF","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1152
builder.connect("CDG","METHOD_REF","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1153
builder.connect("CDG","METHOD_REF","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1154
builder.connect("CDG","METHOD_REF","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1155
builder.connect("CDG","METHOD_REF","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1156
builder.connect("CDG","METHOD_REF","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1157
builder.connect("CDG","METHOD_REF","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1158
builder.connect("CDG","METHOD_REF","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1159
builder.connect("CDG","METHOD_REF","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1160
builder.connect("CDG","TYPE_REF","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1161
builder.connect("CDG","TYPE_REF","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1162
builder.connect("CDG","TYPE_REF","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1163
builder.connect("CDG","TYPE_REF","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1164
builder.connect("CDG","TYPE_REF","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1165
builder.connect("CDG","TYPE_REF","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1166
builder.connect("CDG","TYPE_REF","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1167
builder.connect("CDG","TYPE_REF","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1168
builder.connect("CDG","TYPE_REF","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1169
builder.connect("CDG","TYPE_REF","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1170
builder.connect("CDG","TYPE_REF","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1171
builder.connect("CDG","TYPE_REF","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1172
builder.connect("CDG","JUMP_TARGET","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1173
builder.connect("CDG","JUMP_TARGET","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1174
builder.connect("CDG","JUMP_TARGET","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1175
builder.connect("CDG","JUMP_TARGET","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1176
builder.connect("CDG","JUMP_TARGET","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1177
builder.connect("CDG","JUMP_TARGET","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1178
builder.connect("CDG","JUMP_TARGET","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1179
builder.connect("CDG","JUMP_TARGET","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1180
builder.connect("CDG","JUMP_TARGET","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1181
builder.connect("CDG","JUMP_TARGET","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1182
builder.connect("CDG","JUMP_TARGET","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1183
builder.connect("CDG","JUMP_TARGET","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1184
builder.connect("REACHING_DEF","METHOD","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1185
builder.connect("REACHING_DEF","METHOD","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1186
builder.connect("REACHING_DEF","METHOD","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1187
builder.connect("REACHING_DEF","METHOD","METHOD_PARAMETER_IN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1188
builder.connect("REACHING_DEF","METHOD","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1189
builder.connect("REACHING_DEF","METHOD","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1190
builder.connect("REACHING_DEF","METHOD","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1191
builder.connect("REACHING_DEF","METHOD","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1192
builder.connect("REACHING_DEF","RETURN","METHOD_RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1193
builder.connect("REACHING_DEF","RETURN","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1194
builder.connect("REACHING_DEF","RETURN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1195
builder.connect("REACHING_DEF","RETURN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1196
builder.connect("REACHING_DEF","RETURN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1197
builder.connect("REACHING_DEF","METHOD_REF","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1198
builder.connect("REACHING_DEF","METHOD_REF","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1199
builder.connect("REACHING_DEF","METHOD_REF","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1200
builder.connect("REACHING_DEF","METHOD_REF","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1201
builder.connect("REACHING_DEF","METHOD_REF","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1202
builder.connect("REACHING_DEF","METHOD_REF","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1203
builder.connect("REACHING_DEF","METHOD_REF","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1204
builder.connect("REACHING_DEF","TYPE_REF","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1205
builder.connect("REACHING_DEF","TYPE_REF","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1206
builder.connect("REACHING_DEF","TYPE_REF","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1207
builder.connect("REACHING_DEF","TYPE_REF","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1208
builder.connect("REACHING_DEF","TYPE_REF","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1209
builder.connect("REACHING_DEF","TYPE_REF","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1210
builder.connect("REACHING_DEF","TYPE_REF","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1211
builder.connect("REACHING_DEF","METHOD_PARAMETER_IN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1212
builder.connect("REACHING_DEF","METHOD_PARAMETER_IN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1213
builder.connect("REACHING_DEF","METHOD_PARAMETER_IN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1214
builder.connect("REACHING_DEF","METHOD_PARAMETER_IN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1215
builder.connect("REACHING_DEF","METHOD_PARAMETER_IN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1216
builder.connect("REACHING_DEF","METHOD_PARAMETER_IN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1217
builder.connect("REACHING_DEF","METHOD_PARAMETER_IN","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1218
builder.connect("REACHING_DEF","METHOD_PARAMETER_OUT","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1219
builder.connect("REACHING_DEF","METHOD_PARAMETER_OUT","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1220
builder.connect("REACHING_DEF","METHOD_PARAMETER_OUT","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1221
builder.connect("REACHING_DEF","METHOD_PARAMETER_OUT","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1222
builder.connect("REACHING_DEF","METHOD_PARAMETER_OUT","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1223
builder.connect("REACHING_DEF","METHOD_PARAMETER_OUT","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1224
builder.connect("REACHING_DEF","LITERAL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1225
builder.connect("REACHING_DEF","LITERAL","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1226
builder.connect("REACHING_DEF","LITERAL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1227
builder.connect("REACHING_DEF","LITERAL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1228
builder.connect("REACHING_DEF","LITERAL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1229
builder.connect("REACHING_DEF","LITERAL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1230
builder.connect("REACHING_DEF","LITERAL","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1231
builder.connect("REACHING_DEF","CALL","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1232
builder.connect("REACHING_DEF","CALL","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1233
builder.connect("REACHING_DEF","CALL","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1234
builder.connect("REACHING_DEF","CALL","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1235
builder.connect("REACHING_DEF","CALL","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1236
builder.connect("REACHING_DEF","CALL","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1237
builder.connect("REACHING_DEF","CALL","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1238
builder.connect("REACHING_DEF","IDENTIFIER","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1239
builder.connect("REACHING_DEF","IDENTIFIER","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1240
builder.connect("REACHING_DEF","IDENTIFIER","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1241
builder.connect("REACHING_DEF","IDENTIFIER","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1242
builder.connect("REACHING_DEF","IDENTIFIER","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1243
builder.connect("REACHING_DEF","IDENTIFIER","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1244
builder.connect("REACHING_DEF","IDENTIFIER","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1245
builder.connect("REACHING_DEF","BLOCK","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1246
builder.connect("REACHING_DEF","BLOCK","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1247
builder.connect("REACHING_DEF","BLOCK","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1248
builder.connect("REACHING_DEF","BLOCK","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1249
builder.connect("REACHING_DEF","BLOCK","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1250
builder.connect("REACHING_DEF","BLOCK","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1251
builder.connect("REACHING_DEF","BLOCK","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1252
builder.connect("REACHING_DEF","BLOCK","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1253
builder.connect("REACHING_DEF","CONTROL_STRUCTURE","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1254
builder.connect("REACHING_DEF","CONTROL_STRUCTURE","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1255
builder.connect("REACHING_DEF","CONTROL_STRUCTURE","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1256
builder.connect("REACHING_DEF","CONTROL_STRUCTURE","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1257
builder.connect("REACHING_DEF","CONTROL_STRUCTURE","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1258
builder.connect("REACHING_DEF","CONTROL_STRUCTURE","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1259
builder.connect("REACHING_DEF","CONTROL_STRUCTURE","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1260
builder.connect("REACHING_DEF","UNKNOWN","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1261
builder.connect("REACHING_DEF","UNKNOWN","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1262
builder.connect("REACHING_DEF","UNKNOWN","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1263
builder.connect("REACHING_DEF","UNKNOWN","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1264
builder.connect("REACHING_DEF","UNKNOWN","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1265
builder.connect("REACHING_DEF","UNKNOWN","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1266
builder.connect("REACHING_DEF","UNKNOWN","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1267
builder.add_edge_type("EVAL_TYPE".to_owned(),"This edge connects a node to its evaluation type.",Some(info_13.clone()));
// CPG_CONSTRUCTION_OP:1268
builder.edge_mut("EVAL_TYPE")?.proto_id=Some(21);
// CPG_CONSTRUCTION_OP:1269
builder.add_edge_type("CONTAINS".to_owned(),"This edge connects a node to the method that contains it.",Some(info_13.clone()));
// CPG_CONSTRUCTION_OP:1270
builder.edge_mut("CONTAINS")?.proto_id=Some(28);
// CPG_CONSTRUCTION_OP:1271
builder.add_edge_type("PARAMETER_LINK".to_owned(),"This edge connects a method input parameter to the corresponding\nmethod output parameter.",Some(info_13.clone()));
// CPG_CONSTRUCTION_OP:1272
builder.edge_mut("PARAMETER_LINK")?.proto_id=Some(12);
// CPG_CONSTRUCTION_OP:1273
builder.connect("PARAMETER_LINK","METHOD_PARAMETER_IN","METHOD_PARAMETER_OUT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"asOutput".to_owned(),out_doc:"Traverse to corresponding formal output parameter".to_owned(),in_step:"asInput".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1274
builder.connect("CONTAINS","FILE","TYPE_DECL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1275
builder.connect("CONTAINS","FILE","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1276
builder.connect("CONTAINS","METHOD","CALL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1277
builder.connect("CONTAINS","METHOD","IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1278
builder.connect("CONTAINS","METHOD","FIELD_IDENTIFIER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1279
builder.connect("CONTAINS","METHOD","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"literal".to_owned(),out_doc:"Literals used in the method".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1280
builder.connect("CONTAINS","METHOD","RETURN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1281
builder.connect("CONTAINS","METHOD","METHOD_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1282
builder.connect("CONTAINS","METHOD","TYPE_REF",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1283
builder.connect("CONTAINS","METHOD","BLOCK",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1284
builder.connect("CONTAINS","METHOD","CONTROL_STRUCTURE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1285
builder.connect("CONTAINS","METHOD","JUMP_TARGET",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1286
builder.connect("CONTAINS","METHOD","UNKNOWN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1287
builder.connect("EVAL_TYPE","METHOD_PARAMETER_IN","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::List, out_step:"typ".to_owned(),out_doc:"Traverse to parameter type".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1288
builder.connect("EVAL_TYPE","METHOD_PARAMETER_OUT","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"typ".to_owned(),out_doc:"Traverse to parameter type".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1289
builder.connect("EVAL_TYPE","METHOD_RETURN","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1290
builder.connect("REF","METHOD_REF","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::List, out_step:"referencedMethod".to_owned(),out_doc:"Traverse to referenced method.".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1291
builder.connect("EVAL_TYPE","METHOD_REF","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1292
builder.connect("EVAL_TYPE","TYPE_REF","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1293
builder.connect("REF","TYPE","TYPE_DECL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"referencedTypeDecl".to_owned(),out_doc:"Type declaration which is referenced by this type.".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1294
builder.connect("CONTAINS","TYPE_DECL","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1295
builder.connect("EVAL_TYPE","MEMBER","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"typ".to_owned(),out_doc:"Traverse to member type".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1296
builder.connect("EVAL_TYPE","LITERAL","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1297
builder.connect("REF","CALL","MEMBER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"referencedMember".to_owned(),out_doc:"Traverse to referenced members".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1298
builder.connect("EVAL_TYPE","CALL","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1299
builder.connect("EVAL_TYPE","LOCAL","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"typ".to_owned(),out_doc:"The type of the local.".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1300
builder.connect("EVAL_TYPE","IDENTIFIER","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1301
builder.connect("EVAL_TYPE","BLOCK","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1302
builder.connect("EVAL_TYPE","CONTROL_STRUCTURE","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1303
builder.connect("EVAL_TYPE","UNKNOWN","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1304
builder.add_node_type("COMMENT".to_owned(),"A source code comment",Some(info_14.clone()));
// CPG_CONSTRUCTION_OP:1305
builder.node_mut("COMMENT")?.proto_id=Some(511);
// CPG_CONSTRUCTION_OP:1306
builder.node_mut("COMMENT")?.add_property("FILENAME".to_owned());
// CPG_CONSTRUCTION_OP:1307
builder.node_mut("COMMENT")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:1308
builder.node_mut("COMMENT")?.primary_key=Some("CODE".to_owned());
// CPG_CONSTRUCTION_OP:1309
builder.connect("SOURCE_FILE","COMMENT","COMMENT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"file".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1310
builder.connect("AST","FILE","COMMENT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"comment".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1311
builder.add_edge_type("TAGGED_BY".to_owned(),"Edges from nodes to the tags they are tagged by.",Some(info_15.clone()));
// CPG_CONSTRUCTION_OP:1312
builder.edge_mut("TAGGED_BY")?.proto_id=Some(11);
// CPG_CONSTRUCTION_OP:1313
builder.add_node_type("TAG".to_owned(),"This node represents a tag.",Some(info_15.clone()));
// CPG_CONSTRUCTION_OP:1314
builder.node_mut("TAG")?.proto_id=Some(24);
// CPG_CONSTRUCTION_OP:1315
builder.node_mut("TAG")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:1316
builder.node_mut("TAG")?.add_property("VALUE".to_owned());
// CPG_CONSTRUCTION_OP:1317
builder.node_mut("TAG")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:1318
builder.add_node_type("TAG_NODE_PAIR".to_owned(),"This node contains an arbitrary node and an associated tag node.",Some(info_15.clone()));
// CPG_CONSTRUCTION_OP:1319
builder.node_mut("TAG_NODE_PAIR")?.proto_id=Some(208);
// CPG_CONSTRUCTION_OP:1320
builder.node_mut("TAG_NODE_PAIR")?.contained_nodes.push(ContainedNode{node_type:"TAG".to_owned(),local_name:"tag".to_owned(),cardinality:Cardinality::One { default_wrapper_is_null:false,default:Some(Scalar::Null) },comment:None});
// CPG_CONSTRUCTION_OP:1321
let _=builder.node_mut("AnyNode")?;
// CPG_CONSTRUCTION_OP:1322
builder.node_mut("TAG_NODE_PAIR")?.contained_nodes.push(ContainedNode{node_type:"AnyNode".to_owned(),local_name:"node".to_owned(),cardinality:Cardinality::One { default_wrapper_is_null:false,default:Some(Scalar::Null) },comment:None});
// CPG_CONSTRUCTION_OP:1323
builder.connect("TAGGED_BY","METHOD","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1324
builder.connect("TAGGED_BY","METHOD_RETURN","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1325
builder.connect("TAGGED_BY","LITERAL","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1326
builder.connect("TAGGED_BY","LOCAL","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1327
builder.connect("TAGGED_BY","MEMBER","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1328
builder.connect("TAGGED_BY","CALL","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1329
builder.connect("TAGGED_BY","IDENTIFIER","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1330
builder.connect("TAGGED_BY","FIELD_IDENTIFIER","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1331
builder.connect("TAGGED_BY","METHOD_PARAMETER_IN","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1332
builder.connect("TAGGED_BY","RETURN","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1333
builder.connect("TAGGED_BY","BLOCK","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1334
builder.connect("TAGGED_BY","UNKNOWN","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1335
builder.connect("TAGGED_BY","CONTROL_STRUCTURE","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1336
builder.connect("TAGGED_BY","METHOD_REF","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1337
builder.connect("TAGGED_BY","TYPE_REF","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1338
builder.connect("TAGGED_BY","JUMP_TARGET","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1339
builder.connect("TAGGED_BY","FILE","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1340
builder.connect("TAGGED_BY","METHOD_PARAMETER_OUT","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1341
builder.connect("TAGGED_BY","TAG","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1342
builder.connect("TAGGED_BY","TYPE_DECL","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1343
builder.add_node_type("BINDING".to_owned(),"`BINDING` nodes represent name-signature pairs that can be resolved at a\ntype declaration (`TYPE_DECL`). They are connected to `TYPE_DECL` nodes via\nincoming `BINDS` edges. The bound method is either associated with an outgoing\n`REF` edge to a `METHOD` or with the `METHOD_FULL_NAME` property. The `REF` edge\nif present has priority.",Some(info_16.clone()));
// CPG_CONSTRUCTION_OP:1344
builder.node_mut("BINDING")?.proto_id=Some(146);
// CPG_CONSTRUCTION_OP:1345
builder.node_mut("BINDING")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:1346
builder.node_mut("BINDING")?.add_property("SIGNATURE".to_owned());
// CPG_CONSTRUCTION_OP:1347
builder.node_mut("BINDING")?.add_property("METHOD_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1348
builder.add_edge_type("BINDS".to_owned(),"This edge connects a type declaration (`TYPE_DECL`) with a binding node (`BINDING`) and\nindicates that the type declaration has the binding represented by the binding node, in\nother words, there is a (name, signature) pair that can be resolved for the type\ndeclaration as stored in the binding node.",Some(info_16.clone()));
// CPG_CONSTRUCTION_OP:1349
builder.edge_mut("BINDS")?.proto_id=Some(155);
// CPG_CONSTRUCTION_OP:1350
builder.connect("BINDS","TYPE_DECL","BINDING",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"bindingTypeDecl".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1351
builder.connect("REF","BINDING","METHOD",EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::List, out_step:"boundMethod".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1352
builder.add_node_type("ANNOTATION".to_owned(),"A method annotation.\nThe semantics of the FULL_NAME property on this node differ from the usual FULL_NAME\nsemantics in the sense that FULL_NAME describes the represented annotation class/interface\nitself and not the ANNOTATION node.",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1353
builder.node_mut("ANNOTATION")?.proto_id=Some(5);
// CPG_CONSTRUCTION_OP:1354
builder.node_mut("ANNOTATION")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:1355
builder.node_mut("ANNOTATION")?.add_property("FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1356
builder.node_mut("ANNOTATION")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:1357
builder.add_node_type("ANNOTATION_PARAMETER_ASSIGN".to_owned(),"Assignment of annotation argument to annotation parameter",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1358
builder.node_mut("ANNOTATION_PARAMETER_ASSIGN")?.proto_id=Some(6);
// CPG_CONSTRUCTION_OP:1359
builder.node_mut("ANNOTATION_PARAMETER_ASSIGN")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:1360
builder.add_node_type("ANNOTATION_PARAMETER".to_owned(),"Formal annotation parameter",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1361
builder.node_mut("ANNOTATION_PARAMETER")?.proto_id=Some(7);
// CPG_CONSTRUCTION_OP:1362
builder.node_mut("ANNOTATION_PARAMETER")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:1363
builder.add_node_type("ANNOTATION_LITERAL".to_owned(),"A literal value assigned to an ANNOTATION_PARAMETER",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1364
builder.node_mut("ANNOTATION_LITERAL")?.proto_id=Some(49);
// CPG_CONSTRUCTION_OP:1365
builder.node_mut("ANNOTATION_LITERAL")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:1366
builder.node_mut("ANNOTATION_LITERAL")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:1367
builder.add_node_type("ARRAY_INITIALIZER".to_owned(),"Initialization construct for arrays",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1368
builder.node_mut("ARRAY_INITIALIZER")?.proto_id=Some(14);
// CPG_CONSTRUCTION_OP:1369
builder.node_mut("ARRAY_INITIALIZER")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:1370
builder.node_mut("ARRAY_INITIALIZER")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:1371
builder.connect("AST","ANNOTATION","ANNOTATION_PARAMETER_ASSIGN",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1372
builder.connect("AST","ANNOTATION_PARAMETER_ASSIGN","ANNOTATION_PARAMETER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1373
builder.connect("AST","ANNOTATION_PARAMETER_ASSIGN","ARRAY_INITIALIZER",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1374
builder.connect("AST","ANNOTATION_PARAMETER_ASSIGN","ANNOTATION_LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1375
builder.connect("AST","ANNOTATION_PARAMETER_ASSIGN","ANNOTATION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1376
builder.connect("AST","ARRAY_INITIALIZER","LITERAL",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1377
builder.connect("EVAL_TYPE","ARRAY_INITIALIZER","TYPE",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1378
builder.connect("AST","LITERAL","ANNOTATION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1379
builder.connect("AST","IDENTIFIER","ANNOTATION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1380
builder.connect("AST","METHOD","ANNOTATION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1381
builder.connect("AST","METHOD_PARAMETER_IN","ANNOTATION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1382
builder.connect("AST","METHOD_REF","ANNOTATION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1383
builder.connect("AST","TYPE_DECL","ANNOTATION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1384
builder.connect("AST","MEMBER","ANNOTATION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1385
builder.connect("AST","UNKNOWN","ANNOTATION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1386
builder.add_property("KEY".to_owned(),ValueType::String,"This property denotes a key of a key-value pair.",Some(info_18.clone()));
// CPG_CONSTRUCTION_OP:1387
builder.property_mut("KEY")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:1388
builder.property_mut("KEY")?.proto_id=Some(131);
// CPG_CONSTRUCTION_OP:1389
builder.add_node_type("FINDING".to_owned(),"Finding nodes may be used to store analysis results in the graph\nthat are to be exposed to an end-user, e.g., information about\npotential vulnerabilities or dangerous programming practices.\nA Finding node may contain an abitrary list of key value pairs\nthat characterize the finding, as well as a list of nodes that\nserve as evidence for the finding.",Some(info_18.clone()));
// CPG_CONSTRUCTION_OP:1390
builder.node_mut("FINDING")?.proto_id=Some(214);
// CPG_CONSTRUCTION_OP:1391
builder.add_node_type("KEY_VALUE_PAIR".to_owned(),"This node represents a key value pair, where both the key and the value are strings.",Some(info_18.clone()));
// CPG_CONSTRUCTION_OP:1392
builder.node_mut("KEY_VALUE_PAIR")?.proto_id=Some(217);
// CPG_CONSTRUCTION_OP:1393
builder.node_mut("KEY_VALUE_PAIR")?.add_property("KEY".to_owned());
// CPG_CONSTRUCTION_OP:1394
builder.node_mut("KEY_VALUE_PAIR")?.add_property("VALUE".to_owned());
// CPG_CONSTRUCTION_OP:1395
builder.add_property("EVIDENCE_DESCRIPTION".to_owned(),ValueType::String,"Optional description for nodes in evidence. Used to give a hint about the kind of evidence\nprovided by a node. The evidence description and evidence nodes are associated by index.",Some(info_18.clone()));
// CPG_CONSTRUCTION_OP:1396
builder.property_mut("EVIDENCE_DESCRIPTION")?.as_list();
// CPG_CONSTRUCTION_OP:1397
builder.property_mut("EVIDENCE_DESCRIPTION")?.proto_id=Some(2007162);
// CPG_CONSTRUCTION_OP:1398
builder.node_mut("FINDING")?.contained_nodes.push(ContainedNode{node_type:"AnyNode".to_owned(),local_name:"evidence".to_owned(),cardinality:Cardinality::List,comment:None});
// CPG_CONSTRUCTION_OP:1399
builder.node_mut("FINDING")?.contained_nodes.push(ContainedNode{node_type:"KEY_VALUE_PAIR".to_owned(),local_name:"keyValuePairs".to_owned(),cardinality:Cardinality::List,comment:None});
// CPG_CONSTRUCTION_OP:1400
builder.node_mut("FINDING")?.add_property("EVIDENCE_DESCRIPTION".to_owned());
// CPG_CONSTRUCTION_OP:1401
builder.add_property("CLOSURE_BINDING_ID".to_owned(),ValueType::String,"Identifier which uniquely describes a CLOSURE_BINDING. This property is used to match captured LOCAL nodes with the corresponding CLOSURE_BINDING nodes",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1402
builder.property_mut("CLOSURE_BINDING_ID")?.proto_id=Some(50);
// CPG_CONSTRUCTION_OP:1403
builder.add_edge_type("CAPTURE".to_owned(),"Represents the capturing of a variable into a closure",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1404
builder.edge_mut("CAPTURE")?.proto_id=Some(40);
// CPG_CONSTRUCTION_OP:1405
builder.node_mut("LOCAL")?.add_property("CLOSURE_BINDING_ID".to_owned());
// CPG_CONSTRUCTION_OP:1406
builder.node_mut("METHOD_PARAMETER_IN")?.add_property("CLOSURE_BINDING_ID".to_owned());
// CPG_CONSTRUCTION_OP:1407
builder.add_node_type("CLOSURE_BINDING".to_owned(),"Represents the binding of a LOCAL or METHOD_PARAMETER_IN into the closure of a method",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1408
builder.node_mut("CLOSURE_BINDING")?.proto_id=Some(334);
// CPG_CONSTRUCTION_OP:1409
builder.node_mut("CLOSURE_BINDING")?.add_property("CLOSURE_BINDING_ID".to_owned());
// CPG_CONSTRUCTION_OP:1410
builder.node_mut("CLOSURE_BINDING")?.add_property("EVALUATION_STRATEGY".to_owned());
// CPG_CONSTRUCTION_OP:1411
builder.add_edge_type("CAPTURED_BY".to_owned(),"Connection between a captured LOCAL and the corresponding CLOSURE_BINDING",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1412
builder.edge_mut("CAPTURED_BY")?.proto_id=Some(41);
// CPG_CONSTRUCTION_OP:1413
builder.connect("CAPTURED_BY","LOCAL","CLOSURE_BINDING",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1414
builder.connect("CAPTURED_BY","METHOD_PARAMETER_IN","CLOSURE_BINDING",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1415
builder.connect("CAPTURE","METHOD_REF","CLOSURE_BINDING",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1416
builder.connect("CAPTURE","TYPE_REF","CLOSURE_BINDING",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1417
builder.connect("REF","CLOSURE_BINDING","LOCAL",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1418
builder.connect("REF","CLOSURE_BINDING","METHOD_PARAMETER_IN",EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1419
builder.add_node_type("TEMPLATE_DOM".to_owned(),"This node represents a DOM node used in template languages, e.g., JSX/TSX",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1420
builder.node_mut("TEMPLATE_DOM")?.proto_id=Some(417);
// CPG_CONSTRUCTION_OP:1421
builder.node_mut("TEMPLATE_DOM")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:1422
builder.node_mut("TEMPLATE_DOM")?.extend("EXPRESSION".to_owned());
// CPG_CONSTRUCTION_OP:1423
builder.connect("AST","TEMPLATE_DOM","EXPRESSION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1424
builder.connect("REACHING_DEF","TEMPLATE_DOM","EXPRESSION",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1425
builder.connect("TAGGED_BY","TEMPLATE_DOM","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1426
builder.connect("ARGUMENT","EXPRESSION","TEMPLATE_DOM",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1427
builder.connect("CONTAINS","METHOD","TEMPLATE_DOM",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1428
builder.connect("CONTAINS","FILE","TEMPLATE_DOM",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1429
builder.add_property("DEPENDENCY_GROUP_ID".to_owned(),ValueType::String,"The group ID for a dependency",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1430
builder.property_mut("DEPENDENCY_GROUP_ID")?.proto_id=Some(58);
// CPG_CONSTRUCTION_OP:1431
builder.add_node_type("DEPENDENCY".to_owned(),"This node represents a dependency",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1432
builder.node_mut("DEPENDENCY")?.proto_id=Some(35);
// CPG_CONSTRUCTION_OP:1433
builder.node_mut("DEPENDENCY")?.add_property("VERSION".to_owned());
// CPG_CONSTRUCTION_OP:1434
builder.node_mut("DEPENDENCY")?.add_property("NAME".to_owned());
// CPG_CONSTRUCTION_OP:1435
builder.node_mut("DEPENDENCY")?.add_property("DEPENDENCY_GROUP_ID".to_owned());
// CPG_CONSTRUCTION_OP:1436
builder.node_mut("DEPENDENCY")?.primary_key=Some("NAME".to_owned());
// CPG_CONSTRUCTION_OP:1437
builder.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned(),ValueType::String,"Type hint for the dynamic type. These are observed to be verifiable at runtime.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1438
builder.property_mut("DYNAMIC_TYPE_HINT_FULL_NAME")?.as_list();
// CPG_CONSTRUCTION_OP:1439
builder.property_mut("DYNAMIC_TYPE_HINT_FULL_NAME")?.proto_id=Some(1591);
// CPG_CONSTRUCTION_OP:1440
builder.add_property("POSSIBLE_TYPES".to_owned(),ValueType::String,"Similar to `DYNAMIC_TYPE_HINT_FULL_NAME`, but that this makes no guarantee that types within this property are correct. This property is used to capture observations between node interactions during a 'may-analysis'.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1441
builder.property_mut("POSSIBLE_TYPES")?.as_list();
// CPG_CONSTRUCTION_OP:1442
builder.property_mut("POSSIBLE_TYPES")?.proto_id=Some(1592);
// CPG_CONSTRUCTION_OP:1443
builder.node_mut("CALL")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1444
builder.node_mut("CALL")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1445
builder.node_mut("METHOD_PARAMETER_IN")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1446
builder.node_mut("METHOD_PARAMETER_IN")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1447
builder.node_mut("METHOD_RETURN")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1448
builder.node_mut("METHOD_RETURN")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1449
builder.node_mut("METHOD_REF")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1450
builder.node_mut("METHOD_REF")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1451
builder.node_mut("TYPE_REF")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1452
builder.node_mut("TYPE_REF")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1453
builder.node_mut("LOCAL")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1454
builder.node_mut("LOCAL")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1455
builder.node_mut("BLOCK")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1456
builder.node_mut("BLOCK")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1457
builder.node_mut("UNKNOWN")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1458
builder.node_mut("UNKNOWN")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1459
builder.node_mut("LITERAL")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1460
builder.node_mut("LITERAL")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1461
builder.node_mut("IDENTIFIER")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1462
builder.node_mut("IDENTIFIER")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1463
builder.node_mut("MEMBER")?.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned());
// CPG_CONSTRUCTION_OP:1464
builder.node_mut("MEMBER")?.add_property("POSSIBLE_TYPES".to_owned());
// CPG_CONSTRUCTION_OP:1465
builder.add_node_type("IMPORT".to_owned(),"Declarative import as it is found in statically typed languages like Java.\nThis kind of node is not supposed to be used for imports in dynamically typed\nlanguages like Javascript.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1466
builder.node_mut("IMPORT")?.extend("AST_NODE".to_owned());
// CPG_CONSTRUCTION_OP:1467
builder.node_mut("IMPORT")?.starter_name=Some("imports".to_owned());
// CPG_CONSTRUCTION_OP:1468
builder.add_property("IMPORTED_ENTITY".to_owned(),ValueType::String,"The identifying string of the imported entity.\nFor a Java import like \"import java.nio.ByteBuffer;\" this would be \"java.nio.ByteBuffer\".",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1469
builder.add_property("IMPORTED_AS".to_owned(),ValueType::String,"The identifier under which the import can be accessed in the importing context.\nFor a Java import this is always identical to the class name. But e.g. for a\nKotlin import like \"import java.nio.ByteBuffer as BBuffer\" this would be \"BBuffer\".\nThis property is ignored if IS_WILDCARD is true.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1470
builder.add_property("EXPLICIT_AS".to_owned(),ValueType::Boolean,"Specifies whether the IMPORTED_AS property was explicitly present in the code.\nFor languages like Java which do not allow a renaming during import this is\nalways false. For e.g. Kotlin it depends on the existence of the \"as\" keyword.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1471
builder.add_property("IS_WILDCARD".to_owned(),ValueType::Boolean,"Specifies whether this is a wildcard import.\nFor a Java import like \"import java.nio.*;\" IS_WILDCARD would be \"true\" and\nIMPORTED_ENTITY would be \"java.nio\".\nFor wildcard imports the IMPORTED_AS property is ignored.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1472
builder.add_property("IS_EXPLICIT".to_owned(),ValueType::Boolean,"Specifies whether this is an explicit import.\nMost languages have implicit default imports of some standard library elements\nand this flag is used to distinguish those from explicit imports found in the\ncode base.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1473
builder.add_property("IS_MODULE_IMPORT".to_owned(),ValueType::Boolean,"Specifies whether this is a module import.\nThis is used for languages like Java >= 25 where packages exported by a module\ncan be imported via the module name (which does not need to match the package names in\nany way).",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1474
builder.node_mut("IMPORT")?.add_property("IMPORTED_ENTITY".to_owned());
// CPG_CONSTRUCTION_OP:1475
builder.node_mut("IMPORT")?.add_property("IMPORTED_AS".to_owned());
// CPG_CONSTRUCTION_OP:1476
builder.node_mut("IMPORT")?.add_property("EXPLICIT_AS".to_owned());
// CPG_CONSTRUCTION_OP:1477
builder.node_mut("IMPORT")?.add_property("IS_WILDCARD".to_owned());
// CPG_CONSTRUCTION_OP:1478
builder.node_mut("IMPORT")?.add_property("IS_EXPLICIT".to_owned());
// CPG_CONSTRUCTION_OP:1479
builder.node_mut("IMPORT")?.add_property("IS_MODULE_IMPORT".to_owned());
// CPG_CONSTRUCTION_OP:1480
builder.add_edge_type("IMPORTS".to_owned(),"Edge from imports to dependencies",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1481
builder.edge_mut("IMPORTS")?.proto_id=Some(23663);
// CPG_CONSTRUCTION_OP:1482
builder.add_edge_type("IS_CALL_FOR_IMPORT".to_owned(),"Edge from CALL statement in the AST to the IMPORT.\n￼        |We use this edge to traverse from the logical representation of the IMPORT\n￼        |to the corresponding import statement in the AST.\n￼        |",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1483
builder.edge_mut("IS_CALL_FOR_IMPORT")?.proto_id=Some(23664);
// CPG_CONSTRUCTION_OP:1484
builder.connect("IMPORTS","IMPORT","DEPENDENCY",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1485
builder.connect("TAGGED_BY","IMPORT","TAG",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1486
builder.connect("IS_CALL_FOR_IMPORT","CALL","IMPORT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1487
builder.connect("AST","BLOCK","IMPORT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1488
builder.connect("AST","FILE","IMPORT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1489
builder.connect("AST","TYPE_DECL","IMPORT",EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()})?;
// CPG_CONSTRUCTION_OP:1490
builder.add_property("SECONDARY_ID".to_owned(),ValueType::String,"ID from a different context, e.g. if the graph was imported from a different format,\nwe can use this to preserve the link to the original",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1491
builder.add_property("GENERIC_SIGNATURE".to_owned(),ValueType::String,"This field is experimental. It will likely be removed in the future without any notice.\nIt stores type information for generic types and methods as well as type information\nfor members and locals where the type either contains a type parameter reference or\nan instantiated type reference.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1492
builder.property_mut("GENERIC_SIGNATURE")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:1493
builder.property_mut("GENERIC_SIGNATURE")?.proto_id=Some(3000);
// CPG_CONSTRUCTION_OP:1494
builder.node_mut("METHOD")?.add_property("GENERIC_SIGNATURE".to_owned());
// CPG_CONSTRUCTION_OP:1495
builder.node_mut("TYPE_DECL")?.add_property("GENERIC_SIGNATURE".to_owned());
// CPG_CONSTRUCTION_OP:1496
builder.node_mut("MEMBER")?.add_property("GENERIC_SIGNATURE".to_owned());
// CPG_CONSTRUCTION_OP:1497
builder.node_mut("LOCAL")?.add_property("GENERIC_SIGNATURE".to_owned());
// CPG_CONSTRUCTION_OP:1498
builder.add_property("CONTAINED_REF".to_owned(),ValueType::String,"References to other nodes. This is not a real property; it exists here for the sake of proto serialization only. valueType and cardinality are meaningless.",Some(info_20.clone()));
// CPG_CONSTRUCTION_OP:1499
builder.property_mut("CONTAINED_REF")?.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:1500
builder.property_mut("CONTAINED_REF")?.proto_id=Some(2007161);
// CPG_CONSTRUCTION_OP:1501
builder.node_mut("UNKNOWN")?.add_property("CONTAINED_REF".to_owned());
// CPG_CONSTRUCTION_END
 Ok(())}pub fn schema_parts()->Vec<SchemaInfo>{vec![SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Annotation$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Annotation".to_owned(),schema_name:"Annotation".to_owned(), doc_index:20,description:"\nJava Annotation related CPG definitions.\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Ast$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Ast".to_owned(),schema_name:"Ast".to_owned(), doc_index:7,description:"\nThe Abstract Syntax Tree (AST) Layer provides syntax trees for all compilation units.\nAll nodes of the tree inherit from the same base class (`AST_NODE`) and are connected\nto their child nodes via outgoing `AST` edges.\n\nSyntax trees are typed, that is, when possible, types for all expressions are stored\nin the tree. Moreover, common control structure types are defined in the specification,\nmaking it possible to translate trees into corresponding control flow graphs if only\nthese common control structure types are used, possibly by desugaring on the side of\nthe language frontend. For cases where this is not an option,\nthe AST specification provides means of storing language-dependent information in the\nAST that can be interpreted by language-dependent control flow construction passes.\n\nThis layer MUST be created by the frontend.\n\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Base$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Base".to_owned(),schema_name:"Base".to_owned(), doc_index:2147483647,description:"\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Binding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Binding".to_owned(),schema_name:"Binding".to_owned(), doc_index:19,description:"\nWe use the concept of \"bindings\" to support resolving of\n(method-name, signature) pairs at type declarations (`TYPE_DECL`). For each\npair that we can resolve, we create a `BINDING` node that is connected the\nthe type declaration via an incoming `BINDS` edge. The `BINDING` node is\nconnected to the method it resolves to via an outgoing `REF` edge.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.CallGraph$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.CallGraph".to_owned(),schema_name:"CallGraph".to_owned(), doc_index:8,description:"\nThe Call Graph Layer represents call relations between methods.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Cfg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Cfg".to_owned(),schema_name:"Cfg".to_owned(), doc_index:9,description:"\nThe Control Flow Graph Layer provides control flow graphs for each method. Control\nflow graphs are constructed by marking a sub set of the abstract syntax tree nodes\nas control flow nodes (`CFG_NODE`) and connecting these nodes via `CFG` edges.\nThe control flow graph models both the control flow within the calculation of an\nexpression as well as from expression to expression. The layer can be automatically\ngenerated from the syntax tree layer if only control structure types supported by\nthis specification are employed.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Comment$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Comment".to_owned(),schema_name:"Comment".to_owned(), doc_index:12,description:"".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Configuration$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Configuration".to_owned(),schema_name:"Configuration".to_owned(), doc_index:18,description:"\nThe code property graph specification currently does not contain\nschema elements for the representation of configuration files in\na structured format, however, it does allow configuration files\nto be included verbatim in the graph to enable language-/framework-\nspecific passes to access them. This layer provides the necessary\nschema elements for this basic support of configuration files.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Dominators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Dominators".to_owned(),schema_name:"Dominators".to_owned(), doc_index:10,description:"\nThe Dominators Layer provides dominator- and post-dominator trees for all methods.\nIt is constructed automatically from the control flow graph layer and is in turn\nused to automatically construct control dependence relations of the PDG layer.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.FileSystem$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.FileSystem".to_owned(),schema_name:"FileSystem".to_owned(), doc_index:3,description:"\nCPGs are created from sets of files and the File System Layer describes the layout of\nthese files, that is, it provides information about source files\nand shared objects for source-based and machine-code-based frontends\nrespectively. The purpose of including this information in the CPG\nis to allow nodes of the graph to be mapped back to file system locations.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Finding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Finding".to_owned(),schema_name:"Finding".to_owned(), doc_index:15,description:"\nWe allow findings (e.g., potential vulnerabilities, notes on dangerous practices) to\nbe stored in the Findings Layer.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Hidden$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Hidden".to_owned(),schema_name:"Hidden".to_owned(), doc_index:-1,description:"\nThe schema elements defined here are NOT included on the schema website.\nThese are schema elements that we are using but would like to rework\nin the future and not make part of the standard.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.MetaData$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.MetaData".to_owned(),schema_name:"MetaData".to_owned(), doc_index:2,description:"\nThe Meta Data Layer contains information about CPG creation. In particular,\nit indicates which language frontend generated the CPG and which overlays\nhave been applied. The layer consists of a single node - the Meta Data node -\nand language frontends MUST create this node. Overlay creators MUST edit\nthis node to indicate that a layer has been successfully applied in all\ncases where applying the layer more than once is prohibitive.\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Method$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Method".to_owned(),schema_name:"Method".to_owned(), doc_index:5,description:"\nThe Method Layer contains declarations of methods, functions, and procedures.\nInput parameters and output parameters (including return parameters) are\nrepresented, however, method contents is not present in this layer.\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Namespace$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Namespace".to_owned(),schema_name:"Namespace".to_owned(), doc_index:4,description:"\nMany programming languages allow code to be structured into namespaces. The Namespace\nLayer makes these namespaces explicit and associates program constructs with the\nnamespaces they are defined in.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Operators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Operators".to_owned(),schema_name:"Operators".to_owned(), doc_index:17,description:"\n\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Pdg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Pdg".to_owned(),schema_name:"Pdg".to_owned(), doc_index:11,description:"\n      The Program Dependence Graph Layer contains a program dependence graph for\neach method of the source program. A program dependence graph consists\nof a data dependence graph (DDG) and a control dependence graph (CDG),\ncreated by connecting nodes of the control flow graph via `REACHING_DEF`\nand `CDG` edges respectively.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize".to_owned(),schema_name:"ProtoSerialize".to_owned(), doc_index:2147483647,description:"\n\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Shortcuts$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Shortcuts".to_owned(),schema_name:"Shortcuts".to_owned(), doc_index:16,description:"\nThe Shortcuts Layer provides shortcut edges calculated to speed up\nsubsequent queries. Language frontends MUST NOT create shortcut edges.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Tags$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Tags".to_owned(),schema_name:"Tags".to_owned(), doc_index:17,description:"\nThe Code Property Graph specification allows for tags to be attached\nto arbitrary nodes. Conceptually, this is similar to the creation of\nFinding nodes, however, tags are to be used for intermediate results\nrather than end-results that are to be reported to the user.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Type$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Type".to_owned(),schema_name:"Type".to_owned(), doc_index:6,description:"\nThe Type Layer contains information about type declarations, relations between\ntypes, and type instantiation and usage. In its current form, it allows modelling of\nparametrized types, type hierarchies and aliases.\n".to_owned(),provided_by_frontend:true},]}// CPG_SCHEMA_FACTORY:domain
pub fn domain_schema()->Result<Schema>{let mut builder=SchemaBuilder::new("Cpg".to_owned(),"io.shiftleft.codepropertygraph.generated".to_owned(),vec!["io.shiftleft".to_owned(),]); populate(&mut builder)?;builder.build()}// CPG_SCHEMA_FACTORY:protobuf
pub fn protobuf_schema()->Result<Schema>{let mut builder=SchemaBuilder::new("Cpg".to_owned(),"io.shiftleft.codepropertygraph.generated".to_owned(),vec![]);builder.proto_options(ProtoOptions {package:"cpg".to_owned(),java_outer_classname:"Cpg".to_owned(),java_package:"io.shiftleft.proto.cpg".to_owned(),go_package:"github.com/ShiftLeftSecurity/proto/cpg".to_owned(),csharp_namespace:"io.shiftleft.proto.cpg".to_owned(), uncommon_proto_enum_name_mappings:std::collections::BTreeMap::from([("ControlStructureTypes".to_owned(),"CONTROL_STRUCTURE_TYPES".to_owned()),("Frameworks".to_owned(),"FRAMEWORKS".to_owned()),("Languages".to_owned(),"LANGUAGES".to_owned()),])}); populate(&mut builder)?;builder.build()}// CPG_PROTO_IDS
pub mod proto_ids {// CPG_PROTO_ID:Break
pub const BREAK:i32=1;
// CPG_PROTO_ID:Static
pub const STATIC:i32=1;
// CPG_PROTO_ID:ByReference
pub const BY_REFERENCE:i32=1;
// CPG_PROTO_ID:StaticDispatch
pub const STATIC_DISPATCH:i32=1;
// CPG_PROTO_ID:Java
pub const JAVA:i32=1;
// CPG_PROTO_ID:Method
pub const METHOD:i32=1;
// CPG_PROTO_ID:Public
pub const PUBLIC:i32=2;
// CPG_PROTO_ID:BySharing
pub const BY_SHARING:i32=2;
// CPG_PROTO_ID:Continue
pub const CONTINUE:i32=2;
// CPG_PROTO_ID:DynamicDispatch
pub const DYNAMIC_DISPATCH:i32=2;
// CPG_PROTO_ID:LineNumber
pub const LINE_NUMBER:i32=2;
// CPG_PROTO_ID:JavaScript
pub const JAVA_SCRIPT:i32=2;
// CPG_PROTO_ID:Ast
pub const AST:i32=3;
// CPG_PROTO_ID:Protected
pub const PROTECTED:i32=3;
// CPG_PROTO_ID:While
pub const WHILE:i32=3;
// CPG_PROTO_ID:ParserTypeName
pub const PARSER_TYPE_NAME:i32=3;
// CPG_PROTO_ID:ByValue
pub const BY_VALUE:i32=3;
// CPG_PROTO_ID:Inlined
pub const INLINED:i32=3;
// CPG_PROTO_ID:Golang
pub const GOLANG:i32=3;
// CPG_PROTO_ID:MethodReturn
pub const METHOD_RETURN:i32=3;
// CPG_PROTO_ID:Private
pub const PRIVATE:i32=4;
// CPG_PROTO_ID:Order
pub const ORDER:i32=4;
// CPG_PROTO_ID:Do
pub const DO:i32=4;
// CPG_PROTO_ID:CSharp
pub const C_SHARP:i32=4;
// CPG_PROTO_ID:Abstract
pub const ABSTRACT:i32=5;
// CPG_PROTO_ID:Annotation
pub const ANNOTATION:i32=5;
// CPG_PROTO_ID:For
pub const FOR:i32=5;
// CPG_PROTO_ID:Name
pub const NAME:i32=5;
// CPG_PROTO_ID:C
pub const C:i32=5;
// CPG_PROTO_ID:FullName
pub const FULL_NAME:i32=6;
// CPG_PROTO_ID:AnnotationParameterAssign
pub const ANNOTATION_PARAMETER_ASSIGN:i32=6;
// CPG_PROTO_ID:Goto
pub const GOTO:i32=6;
// CPG_PROTO_ID:Native
pub const NATIVE:i32=6;
// CPG_PROTO_ID:CallEdge
pub const CALL_EDGE:i32=6;
// CPG_PROTO_ID:Python
pub const PYTHON:i32=6;
// CPG_PROTO_ID:AnnotationParameter
pub const ANNOTATION_PARAMETER:i32=7;
// CPG_PROTO_ID:If
pub const IF:i32=7;
// CPG_PROTO_ID:Constructor
pub const CONSTRUCTOR:i32=7;
// CPG_PROTO_ID:IsExternal
pub const IS_EXTERNAL:i32=7;
// CPG_PROTO_ID:LLVM
pub const LLVM:i32=7;
// CPG_PROTO_ID:Virtual
pub const VIRTUAL:i32=8;
// CPG_PROTO_ID:Value
pub const VALUE:i32=8;
// CPG_PROTO_ID:Else
pub const ELSE:i32=8;
// CPG_PROTO_ID:Literal
pub const LITERAL:i32=8;
// CPG_PROTO_ID:PHP
pub const PHP:i32=8;
// CPG_PROTO_ID:Internal
pub const INTERNAL:i32=9;
// CPG_PROTO_ID:Switch
pub const SWITCH:i32=9;
// CPG_PROTO_ID:FuzzyTestLang
pub const FUZZY_TEST_LANG:i32=9;
// CPG_PROTO_ID:Member
pub const MEMBER:i32=9;
// CPG_PROTO_ID:Final
pub const FINAL:i32=10;
// CPG_PROTO_ID:Try
pub const TRY:i32=10;
// CPG_PROTO_ID:Ref
pub const REF:i32=10;
// CPG_PROTO_ID:Ghidra
pub const GHIDRA:i32=10;
// CPG_PROTO_ID:Kotlin
pub const KOTLIN:i32=11;
// CPG_PROTO_ID:ReadOnly
pub const READ_ONLY:i32=11;
// CPG_PROTO_ID:Throw
pub const THROW:i32=11;
// CPG_PROTO_ID:ColumnNumber
pub const COLUMN_NUMBER:i32=11;
// CPG_PROTO_ID:Variable
pub const VARIABLE:i32=11;
// CPG_PROTO_ID:TaggedBy
pub const TAGGED_BY:i32=11;
// CPG_PROTO_ID:NewC
pub const NEW_C:i32=12;
// CPG_PROTO_ID:LineNumberEnd
pub const LINE_NUMBER_END:i32=12;
// CPG_PROTO_ID:Module
pub const MODULE:i32=12;
// CPG_PROTO_ID:Match
pub const MATCH:i32=12;
// CPG_PROTO_ID:ParameterLink
pub const PARAMETER_LINK:i32=12;
// CPG_PROTO_ID:JavaSrc
pub const JAVA_SRC:i32=13;
// CPG_PROTO_ID:Lambda
pub const LAMBDA:i32=13;
// CPG_PROTO_ID:Version
pub const VERSION:i32=13;
// CPG_PROTO_ID:Yield
pub const YIELD:i32=13;
// CPG_PROTO_ID:PythonSrc
pub const PYTHON_SRC:i32=14;
// CPG_PROTO_ID:ArrayInitializer
pub const ARRAY_INITIALIZER:i32=14;
// CPG_PROTO_ID:Catch
pub const CATCH:i32=14;
// CPG_PROTO_ID:Finally
pub const FINALLY:i32=15;
// CPG_PROTO_ID:CallNode
pub const CALL_NODE:i32=15;
// CPG_PROTO_ID:EvaluationStrategy
pub const EVALUATION_STRATEGY:i32=15;
// CPG_PROTO_ID:JsSrc
pub const JS_SRC:i32=15;
// CPG_PROTO_ID:ColumnNumberEnd
pub const COLUMN_NUMBER_END:i32=16;
// CPG_PROTO_ID:RubySrc
pub const RUBY_SRC:i32=17;
// CPG_PROTO_ID:SwiftSrc
pub const SWIFT_SRC:i32=18;
// CPG_PROTO_ID:Cfg
pub const CFG:i32=19;
// CPG_PROTO_ID:Language
pub const LANGUAGE:i32=19;
// CPG_PROTO_ID:CSharpSrc
pub const C_SHARP_SRC:i32=19;
// CPG_PROTO_ID:ABAP
pub const ABAP:i32=20;
// CPG_PROTO_ID:Rust
pub const RUST:i32=21;
// CPG_PROTO_ID:Content
pub const CONTENT:i32=20;
// CPG_PROTO_ID:Code
pub const CODE:i32=21;
// CPG_PROTO_ID:EvalType
pub const EVAL_TYPE:i32=21;
// CPG_PROTO_ID:BindsTo
pub const BINDS_TO:i32=22;
// CPG_PROTO_ID:Signature
pub const SIGNATURE:i32=22;
// CPG_PROTO_ID:Local
pub const LOCAL:i32=23;
// CPG_PROTO_ID:InheritsFrom
pub const INHERITS_FROM:i32=23;
// CPG_PROTO_ID:Tag
pub const TAG:i32=24;
// CPG_PROTO_ID:Location
pub const LOCATION:i32=25;
// CPG_PROTO_ID:DispatchType
pub const DISPATCH_TYPE:i32=25;
// CPG_PROTO_ID:ModifierType
pub const MODIFIER_TYPE:i32=26;
// CPG_PROTO_ID:Identifier
pub const IDENTIFIER:i32=27;
// CPG_PROTO_ID:ControlStructureType
pub const CONTROL_STRUCTURE_TYPE:i32=27;
// CPG_PROTO_ID:Contains
pub const CONTAINS:i32=28;
// CPG_PROTO_ID:Return
pub const RETURN:i32=30;
// CPG_PROTO_ID:Block
pub const BLOCK:i32=31;
// CPG_PROTO_ID:MethodParameterOut
pub const METHOD_PARAMETER_OUT:i32=33;
// CPG_PROTO_ID:MethodParameterIn
pub const METHOD_PARAMETER_IN:i32=34;
// CPG_PROTO_ID:Dependency
pub const DEPENDENCY:i32=35;
// CPG_PROTO_ID:File
pub const FILE:i32=38;
// CPG_PROTO_ID:MetaData
pub const META_DATA:i32=39;
// CPG_PROTO_ID:ArgumentIndex
pub const ARGUMENT_INDEX:i32=40;
// CPG_PROTO_ID:Capture
pub const CAPTURE:i32=40;
// CPG_PROTO_ID:Namespace
pub const NAMESPACE:i32=40;
// CPG_PROTO_ID:CapturedBy
pub const CAPTURED_BY:i32=41;
// CPG_PROTO_ID:NamespaceBlock
pub const NAMESPACE_BLOCK:i32=41;
// CPG_PROTO_ID:Unknown
pub const UNKNOWN:i32=44;
// CPG_PROTO_ID:Type
pub const TYPE:i32=45;
// CPG_PROTO_ID:TypeDecl
pub const TYPE_DECL:i32=46;
// CPG_PROTO_ID:TypeParameter
pub const TYPE_PARAMETER:i32=47;
// CPG_PROTO_ID:TypeArgument
pub const TYPE_ARGUMENT:i32=48;
// CPG_PROTO_ID:AnnotationLiteral
pub const ANNOTATION_LITERAL:i32=49;
// CPG_PROTO_ID:ConfigFile
pub const CONFIG_FILE:i32=50;
// CPG_PROTO_ID:ClosureBindingId
pub const CLOSURE_BINDING_ID:i32=50;
// CPG_PROTO_ID:TypeFullName
pub const TYPE_FULL_NAME:i32=51;
// CPG_PROTO_ID:TypeDeclFullName
pub const TYPE_DECL_FULL_NAME:i32=52;
// CPG_PROTO_ID:InheritsFromTypeFullName
pub const INHERITS_FROM_TYPE_FULL_NAME:i32=53;
// CPG_PROTO_ID:MethodFullName
pub const METHOD_FULL_NAME:i32=54;
// CPG_PROTO_ID:Receiver
pub const RECEIVER:i32=55;
// CPG_PROTO_ID:Condition
pub const CONDITION:i32=56;
// CPG_PROTO_ID:AstParentType
pub const AST_PARENT_TYPE:i32=56;
// CPG_PROTO_ID:AstParentFullName
pub const AST_PARENT_FULL_NAME:i32=57;
// CPG_PROTO_ID:DependencyGroupId
pub const DEPENDENCY_GROUP_ID:i32=58;
// CPG_PROTO_ID:StaticReceiver
pub const STATIC_RECEIVER:i32=59;
// CPG_PROTO_ID:TrueBody
pub const TRUE_BODY:i32=60;
// CPG_PROTO_ID:FalseBody
pub const FALSE_BODY:i32=61;
// CPG_PROTO_ID:DoBody
pub const DO_BODY:i32=62;
// CPG_PROTO_ID:ForInit
pub const FOR_INIT:i32=63;
// CPG_PROTO_ID:ForUpdate
pub const FOR_UPDATE:i32=64;
// CPG_PROTO_ID:ForBody
pub const FOR_BODY:i32=65;
// CPG_PROTO_ID:TryBody
pub const TRY_BODY:i32=66;
// CPG_PROTO_ID:CatchBody
pub const CATCH_BODY:i32=67;
// CPG_PROTO_ID:FinallyBody
pub const FINALLY_BODY:i32=68;
// CPG_PROTO_ID:JumpArgument
pub const JUMP_ARGUMENT:i32=69;
// CPG_PROTO_ID:Symbol
pub const SYMBOL:i32=100;
// CPG_PROTO_ID:MethodShortName
pub const METHOD_SHORT_NAME:i32=102;
// CPG_PROTO_ID:PackageName
pub const PACKAGE_NAME:i32=103;
// CPG_PROTO_ID:ClassName
pub const CLASS_NAME:i32=104;
// CPG_PROTO_ID:NodeLabel
pub const NODE_LABEL:i32=105;
// CPG_PROTO_ID:FileName
pub const FILE_NAME:i32=106;
// CPG_PROTO_ID:Overlays
pub const OVERLAYS:i32=118;
// CPG_PROTO_ID:Hash
pub const HASH:i32=120;
// CPG_PROTO_ID:ArgumentLabel
pub const ARGUMENT_LABEL:i32=129;
// CPG_PROTO_ID:ArgumentName
pub const ARGUMENT_NAME:i32=130;
// CPG_PROTO_ID:Key
pub const KEY:i32=131;
// CPG_PROTO_ID:ClassShortName
pub const CLASS_SHORT_NAME:i32=132;
// CPG_PROTO_ID:ReachingDef
pub const REACHING_DEF:i32=137;
// CPG_PROTO_ID:AliasOf
pub const ALIAS_OF:i32=138;
// CPG_PROTO_ID:Binding
pub const BINDING:i32=146;
// CPG_PROTO_ID:Binds
pub const BINDS:i32=155;
// CPG_PROTO_ID:Argument
pub const ARGUMENT:i32=156;
// CPG_PROTO_ID:SourceFile
pub const SOURCE_FILE:i32=157;
// CPG_PROTO_ID:AliasTypeFullName
pub const ALIAS_TYPE_FULL_NAME:i32=158;
// CPG_PROTO_ID:Dominate
pub const DOMINATE:i32=181;
// CPG_PROTO_ID:PostDominate
pub const POST_DOMINATE:i32=182;
// CPG_PROTO_ID:Cdg
pub const CDG:i32=183;
// CPG_PROTO_ID:TagNodePair
pub const TAG_NODE_PAIR:i32=208;
// CPG_PROTO_ID:Finding
pub const FINDING:i32=214;
// CPG_PROTO_ID:KeyValuePair
pub const KEY_VALUE_PAIR:i32=217;
// CPG_PROTO_ID:IsVariadic
pub const IS_VARIADIC:i32=221;
// CPG_PROTO_ID:Modifier
pub const MODIFIER:i32=300;
// CPG_PROTO_ID:MethodRef
pub const METHOD_REF:i32=333;
// CPG_PROTO_ID:ClosureBinding
pub const CLOSURE_BINDING:i32=334;
// CPG_PROTO_ID:TypeRef
pub const TYPE_REF:i32=335;
// CPG_PROTO_ID:ControlStructure
pub const CONTROL_STRUCTURE:i32=339;
// CPG_PROTO_ID:JumpTarget
pub const JUMP_TARGET:i32=340;
// CPG_PROTO_ID:JumpLabel
pub const JUMP_LABEL:i32=341;
// CPG_PROTO_ID:TemplateDom
pub const TEMPLATE_DOM:i32=417;
// CPG_PROTO_ID:Comment
pub const COMMENT:i32=511;
// CPG_PROTO_ID:Root
pub const ROOT:i32=1199;
// CPG_PROTO_ID:DynamicTypeHintFullName
pub const DYNAMIC_TYPE_HINT_FULL_NAME:i32=1591;
// CPG_PROTO_ID:PossibleTypes
pub const POSSIBLE_TYPES:i32=1592;
// CPG_PROTO_ID:Index
pub const INDEX:i32=2223;
// CPG_PROTO_ID:GenericSignature
pub const GENERIC_SIGNATURE:i32=3000;
// CPG_PROTO_ID:Offset
pub const OFFSET:i32=3812;
// CPG_PROTO_ID:OffsetEnd
pub const OFFSET_END:i32=3813;
// CPG_PROTO_ID:Imports
pub const IMPORTS:i32=23663;
// CPG_PROTO_ID:IsCallForImport
pub const IS_CALL_FOR_IMPORT:i32=23664;
// CPG_PROTO_ID:FieldIdentifier
pub const FIELD_IDENTIFIER:i32=2001081;
// CPG_PROTO_ID:CanonicalName
pub const CANONICAL_NAME:i32=2001092;
// CPG_PROTO_ID:ContainedRef
pub const CONTAINED_REF:i32=2007161;
// CPG_PROTO_ID:EvidenceDescription
pub const EVIDENCE_DESCRIPTION:i32=2007162;
}