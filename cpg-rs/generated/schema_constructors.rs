// Generated native CPG schema construction. Do not edit.
use crate::schema::*;use crate::Result;// CPG_SCHEMA_MODULE:Base
#[derive(Clone,Debug)] pub struct BaseSchema {pub version:PropertyRef,pub hash:PropertyRef,pub code:PropertyRef,pub is_external:PropertyRef,pub index:PropertyRef,pub name:PropertyRef,pub full_name:PropertyRef,pub parser_type_name:PropertyRef,pub value:PropertyRef,pub content:PropertyRef,pub ast_parent_type:PropertyRef,pub ast_parent_full_name:PropertyRef,pub declaration:NodeRef,pub r#ref:EdgeRef,}impl BaseSchema { pub fn new(builder:&mut ObjectSchemaBuilder,)->Result<Self>{let info_0=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Base$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Base".to_owned(),schema_name:"Base".to_owned(), doc_index:2147483647,description:"\n".to_owned(),provided_by_frontend:true};// CPG_CONSTRUCTION_OP:1
let _object_1=builder.add_property("VERSION".to_owned(),ValueType::String,"A version, given as a string. Used, for example, in the META_DATA node to\nindicate which version of the CPG spec this CPG conforms to",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:2
_object_1.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:3
_object_1.proto_id(13);
// CPG_CONSTRUCTION_OP:4
let _object_2=builder.add_property("HASH".to_owned(),ValueType::String,"This property contains a hash value in the form of a string.\nHashes can be used to summarize data, e.g., to summarize the\ncontents of source files or sub graphs. Such summaries are useful\nto determine whether code has already been analyzed in incremental\nanalysis pipelines. This property is optional to allow its calculation\nto be deferred or skipped if the hash is not needed.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:5
_object_2.proto_id(120);
// CPG_CONSTRUCTION_OP:6
let _object_3=builder.add_property("CODE".to_owned(),ValueType::String,"This field holds the code snippet that the node represents.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:7
_object_3.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:8
_object_3.proto_id(21);
// CPG_CONSTRUCTION_OP:9
let _object_4=builder.add_property("IS_EXTERNAL".to_owned(),ValueType::Boolean,"Indicates that the construct (METHOD or TYPE_DECL) is external, that is,\nit is referenced but not defined in the code (applies both to insular\nparsing and to library functions where we have header files only)",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:10
_object_4.mandatory(Scalar::Boolean { value:false })?;
// CPG_CONSTRUCTION_OP:11
_object_4.proto_id(7);
// CPG_CONSTRUCTION_OP:12
let _object_5=builder.add_property("INDEX".to_owned(),ValueType::Int,"Specifies an index, e.g., for a parameter or argument.\n Explicit parameters are numbered from 1 to N, while index 0 is reserved for implicit\n self / this parameter.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:13
_object_5.mandatory(Scalar::Int { decimal:"-2".to_owned() })?;
// CPG_CONSTRUCTION_OP:14
_object_5.proto_id(2223);
// CPG_CONSTRUCTION_OP:15
let _object_6=builder.add_property("NAME".to_owned(),ValueType::String,"Name of represented object, e.g., method name (e.g. \"run\")",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:16
_object_6.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:17
_object_6.proto_id(5);
// CPG_CONSTRUCTION_OP:18
let _object_7=builder.add_property("FULL_NAME".to_owned(),ValueType::String,"This is the fully-qualified name of an entity, e.g., the fully-qualified\nname of a method or type. The details of what constitutes a fully-qualified\nname are language specific. This field SHOULD be human readable.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:19
_object_7.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:20
_object_7.proto_id(6);
// CPG_CONSTRUCTION_OP:21
let _object_8=builder.add_property("PARSER_TYPE_NAME".to_owned(),ValueType::String,"AST node type name emitted by parser.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:22
_object_8.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:23
_object_8.proto_id(3);
// CPG_CONSTRUCTION_OP:24
let _object_9=builder.add_property("VALUE".to_owned(),ValueType::String,"This property denotes a string value as used in a key-value pair.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:25
_object_9.mandatory(Scalar::String { value:"".to_owned() })?;
// CPG_CONSTRUCTION_OP:26
_object_9.proto_id(8);
// CPG_CONSTRUCTION_OP:27
let _object_10=builder.add_property("CONTENT".to_owned(),ValueType::String,"Certain files, e.g., configuration files, may be included in the CPG as-is.\nFor such files, the `CONTENT` field contains the files content.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:28
_object_10.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:29
_object_10.proto_id(20);
// CPG_CONSTRUCTION_OP:30
let _object_11=builder.add_property("AST_PARENT_TYPE".to_owned(),ValueType::String,"The type of the AST parent. Since this is only used in some parts of the graph,\nthe list does not include all possible parents by intention.\nPossible parents: METHOD, TYPE_DECL, NAMESPACE_BLOCK.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:31
_object_11.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:32
_object_11.proto_id(56);
// CPG_CONSTRUCTION_OP:33
let _object_12=builder.add_property("AST_PARENT_FULL_NAME".to_owned(),ValueType::String,"This field holds the FULL_NAME of the AST parent of an entity.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:34
_object_12.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:35
_object_12.proto_id(57);
// CPG_CONSTRUCTION_OP:36
let _object_13=builder.add_node_base_type("DECLARATION".to_owned(),"This is the base node class for all declarations.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:37
_object_13.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:38
let _object_14=builder.add_edge_type("REF".to_owned(),"This edge indicates that the source node is an identifier that denotes\naccess to the destination node. For example, an identifier may reference\na local variable.",Some(info_0.clone()));
// CPG_CONSTRUCTION_OP:39
_object_14.proto_id(10);
 Ok(Self{version:_object_1,hash:_object_2,code:_object_3,is_external:_object_4,index:_object_5,name:_object_6,full_name:_object_7,parser_type_name:_object_8,value:_object_9,content:_object_10,ast_parent_type:_object_11,ast_parent_full_name:_object_12,declaration:_object_13,r#ref:_object_14,})} pub fn apply(builder:&mut ObjectSchemaBuilder,)->Result<Self>{Self::new(builder,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Base$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Base".to_owned(),schema_name:"Base".to_owned(), doc_index:2147483647,description:"\n".to_owned(),provided_by_frontend:true}} } impl SchemaBase for BaseSchema { fn doc_index(&self)->i32{2147483647} fn description(&self)->&str{"\n"} fn provided_by_frontend(&self)->bool{true} }// CPG_SCHEMA_MODULE_END:Base
// CPG_SCHEMA_MODULE:FileSystem
#[derive(Clone,Debug)] pub struct FileSystemSchema {pub filename:PropertyRef,pub line_number:PropertyRef,pub column_number:PropertyRef,pub line_number_end:PropertyRef,pub column_number_end:PropertyRef,pub source_file:EdgeRef,pub file:NodeRef,pub offset:PropertyRef,pub offset_end:PropertyRef,}impl FileSystemSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,)->Result<Self>{let _object_10=_arg_base.content.clone();let _object_2=_arg_base.hash.clone();let _object_6=_arg_base.name.clone();let info_1=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.FileSystem$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.FileSystem".to_owned(),schema_name:"FileSystem".to_owned(), doc_index:3,description:"\nCPGs are created from sets of files and the File System Layer describes the layout of\nthese files, that is, it provides information about source files\nand shared objects for source-based and machine-code-based frontends\nrespectively. The purpose of including this information in the CPG\nis to allow nodes of the graph to be mapped back to file system locations.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:40
let _object_15=builder.add_property("FILENAME".to_owned(),ValueType::String,"The path of the source file this node was generated from, relative to the root\npath in the meta data node. This field must be set but may be set to the value `<unknown>` to\nindicate that no source file can be associated with the node, e.g., because the node represents\nan entity known to exist because it is referenced, but for which the file that is is declared in\nis unknown.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:41
_object_15.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:42
_object_15.proto_id(106);
// CPG_CONSTRUCTION_OP:43
let _object_16=builder.add_property("LINE_NUMBER".to_owned(),ValueType::Int,"This optional field provides the line number of the program construct\nrepresented by the node.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:44
_object_16.proto_id(2);
// CPG_CONSTRUCTION_OP:45
let _object_17=builder.add_property("COLUMN_NUMBER".to_owned(),ValueType::Int,"This optional fields provides the column number of the program construct\nrepresented by the node.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:46
_object_17.proto_id(11);
// CPG_CONSTRUCTION_OP:47
let _object_18=builder.add_property("LINE_NUMBER_END".to_owned(),ValueType::Int,"This optional fields provides the line number at which the program construct\nrepresented by the node ends.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:48
_object_18.proto_id(12);
// CPG_CONSTRUCTION_OP:49
let _object_19=builder.add_property("COLUMN_NUMBER_END".to_owned(),ValueType::Int,"This optional fields provides the column number at which the program construct\nrepresented by the node ends.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:50
_object_19.proto_id(16);
// CPG_CONSTRUCTION_OP:51
let _object_20=builder.add_edge_type("SOURCE_FILE".to_owned(),"This edge connects a node to the node that represents its source file. These\nedges MUST not be created by the language frontend but are automatically\ncreated based on `FILENAME` fields.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:52
_object_20.proto_id(157);
// CPG_CONSTRUCTION_OP:53
let _object_21=builder.add_node_type("FILE".to_owned(),"File nodes represent source files or a shared objects from which the CPG\nwas generated. File nodes serve as indices, that is, they allow looking up all\nelements of the code by file.\n\nFor each file, the graph CAN contain exactly one File node, if not File nodes\nare created as indicated by `FILENAME` property of other nodes.\nAs file nodes are root nodes of abstract syntax tress, they are AstNodes and\ntheir order field is set to 0. This is because they have no sibling nodes,\nnot because they are the first node of the AST.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:54
_object_21.proto_id(38);
// CPG_CONSTRUCTION_OP:55
_object_21.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:56
_object_21.add_property(&_object_2);
// CPG_CONSTRUCTION_OP:57
_object_21.add_property(&_object_10);
// CPG_CONSTRUCTION_OP:58
_object_21.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:59
let _object_22=builder.add_property("OFFSET".to_owned(),ValueType::Int,"Start offset into the CONTENT property of the corresponding FILE node.\nThe offset is such that parts of the content can easily\nbe accessed via `content.substring(offset, offsetEnd)`.\nThis means that the offset must be measured in utf16 encoding (i.e. neither in\ncharacters/codeunits nor in byte-offsets into a utf8 encoding).\nE.g. for METHOD nodes this start offset points to the start of the methods\nsource code in the string holding the source code of the entire file.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:60
_object_22.proto_id(3812);
// CPG_CONSTRUCTION_OP:61
let _object_23=builder.add_property("OFFSET_END".to_owned(),ValueType::Int,"End offset (exclusive) into the CONTENT property of the corresponding FILE node.\nSee OFFSET documentation for finer details.\nE.g. for METHOD nodes this end offset points to the first code position which is\nnot part of the method.",Some(info_1.clone()));
// CPG_CONSTRUCTION_OP:62
_object_23.proto_id(3813);
 Ok(Self{filename:_object_15,line_number:_object_16,column_number:_object_17,line_number_end:_object_18,column_number_end:_object_19,source_file:_object_20,file:_object_21,offset:_object_22,offset_end:_object_23,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,)->Result<Self>{Self::new(builder,_arg_base,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.FileSystem$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.FileSystem".to_owned(),schema_name:"FileSystem".to_owned(), doc_index:3,description:"\nCPGs are created from sets of files and the File System Layer describes the layout of\nthese files, that is, it provides information about source files\nand shared objects for source-based and machine-code-based frontends\nrespectively. The purpose of including this information in the CPG\nis to allow nodes of the graph to be mapped back to file system locations.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for FileSystemSchema { fn doc_index(&self)->i32{3} fn description(&self)->&str{"\nCPGs are created from sets of files and the File System Layer describes the layout of\nthese files, that is, it provides information about source files\nand shared objects for source-based and machine-code-based frontends\nrespectively. The purpose of including this information in the CPG\nis to allow nodes of the graph to be mapped back to file system locations.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:FileSystem
// CPG_SCHEMA_MODULE:Namespace
#[derive(Clone,Debug)] pub struct NamespaceSchema {pub namespace_block:NodeRef,pub namespace:NodeRef,}impl NamespaceSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{let _object_15=_arg_fs.filename.clone();let _object_20=_arg_fs.source_file.clone();let _object_21=_arg_fs.file.clone();let _object_6=_arg_base.name.clone();let _object_7=_arg_base.full_name.clone();let info_2=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Namespace$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Namespace".to_owned(),schema_name:"Namespace".to_owned(), doc_index:4,description:"\nMany programming languages allow code to be structured into namespaces. The Namespace\nLayer makes these namespaces explicit and associates program constructs with the\nnamespaces they are defined in.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:63
let _object_24=builder.add_node_type("NAMESPACE_BLOCK".to_owned(),"A reference to a namespace.\nWe borrow the concept of a \"namespace block\" from C++, that is, a namespace block\nis a block of code that has been placed in the same namespace by a programmer.\nThis block may be introduced via a `package` statement in Java or\na `namespace{ }` statement in C++.\n\nThe `FULL_NAME` field contains a unique identifier to represent the namespace block\nitself not just the namespace it references. So in addition to the namespace name\nit can be useful to use the containing file name to derive a unique identifier.\n\nThe `NAME` field contains the namespace name in a human-readable format.\nThe name should be given in dot-separated form where a dot indicates\nthat the right hand side is a sub namespace of the left hand side, e.g.,\n`foo.bar` denotes the namespace `bar` contained in the namespace `foo`.",Some(info_2.clone()));
// CPG_CONSTRUCTION_OP:64
_object_24.proto_id(41);
// CPG_CONSTRUCTION_OP:65
_object_24.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:66
_object_24.add_property(&_object_7);
// CPG_CONSTRUCTION_OP:67
_object_24.add_property(&_object_15);
// CPG_CONSTRUCTION_OP:68
_object_24.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:69
let _object_25=builder.add_node_type("NAMESPACE".to_owned(),"This node represents a namespace. Similar to FILE nodes, NAMESPACE nodes\nserve as indices that allow all definitions inside a namespace to be\nobtained by following outgoing edges from a NAMESPACE node.\n\nNAMESPACE nodes MUST NOT be created by language frontends. Instead,\nthey are generated from NAMESPACE_BLOCK nodes automatically upon\nfirst loading of the CPG.",Some(info_2.clone()));
// CPG_CONSTRUCTION_OP:70
_object_25.proto_id(40);
// CPG_CONSTRUCTION_OP:71
_object_25.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:72
_object_25.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:73
_object_24.add_out_edge(&_object_20,&_object_21,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"namespaceBlock".to_owned(),in_doc:"".to_owned()});
 Ok(Self{namespace_block:_object_24,namespace:_object_25,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{Self::new(builder,_arg_base,_arg_fs,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Namespace$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Namespace".to_owned(),schema_name:"Namespace".to_owned(), doc_index:4,description:"\nMany programming languages allow code to be structured into namespaces. The Namespace\nLayer makes these namespaces explicit and associates program constructs with the\nnamespaces they are defined in.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for NamespaceSchema { fn doc_index(&self)->i32{4} fn description(&self)->&str{"\nMany programming languages allow code to be structured into namespaces. The Namespace\nLayer makes these namespaces explicit and associates program constructs with the\nnamespaces they are defined in.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Namespace
// CPG_SCHEMA_MODULE:Operators
#[derive(Clone,Debug)] pub struct OperatorsSchema {pub operators:Vec<ConstantRef>,}impl OperatorsSchema { pub fn new(builder:&mut ObjectSchemaBuilder,)->Result<Self>{let info_3=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Operators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Operators".to_owned(),schema_name:"Operators".to_owned(), doc_index:17,description:"\n\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:74
let constant_26=builder.constant(Constant{name:"addition".to_owned(),value:"<operator>.addition".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:75
let constant_27=builder.constant(Constant{name:"subtraction".to_owned(),value:"<operator>.subtraction".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:76
let constant_28=builder.constant(Constant{name:"multiplication".to_owned(),value:"<operator>.multiplication".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:77
let constant_29=builder.constant(Constant{name:"division".to_owned(),value:"<operator>.division".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:78
let constant_30=builder.constant(Constant{name:"exponentiation".to_owned(),value:"<operator>.exponentiation".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:79
let constant_31=builder.constant(Constant{name:"modulo".to_owned(),value:"<operator>.modulo".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:80
let constant_32=builder.constant(Constant{name:"shiftLeft".to_owned(),value:"<operator>.shiftLeft".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:81
let constant_33=builder.constant(Constant{name:"logicalShiftRight".to_owned(),value:"<operator>.logicalShiftRight".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Shift right padding with zeros".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:82
let constant_34=builder.constant(Constant{name:"arithmeticShiftRight".to_owned(),value:"<operator>.arithmeticShiftRight".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Shift right preserving the sign".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:83
let constant_35=builder.constant(Constant{name:"not".to_owned(),value:"<operator>.not".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:84
let constant_36=builder.constant(Constant{name:"and".to_owned(),value:"<operator>.and".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:85
let constant_37=builder.constant(Constant{name:"or".to_owned(),value:"<operator>.or".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:86
let constant_38=builder.constant(Constant{name:"xor".to_owned(),value:"<operator>.xor".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:87
let constant_39=builder.constant(Constant{name:"assignmentPlus".to_owned(),value:"<operator>.assignmentPlus".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:88
let constant_40=builder.constant(Constant{name:"assignmentMinus".to_owned(),value:"<operator>.assignmentMinus".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:89
let constant_41=builder.constant(Constant{name:"assignmentMultiplication".to_owned(),value:"<operator>.assignmentMultiplication".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:90
let constant_42=builder.constant(Constant{name:"assignmentDivision".to_owned(),value:"<operator>.assignmentDivision".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:91
let constant_43=builder.constant(Constant{name:"assignmentExponentiation".to_owned(),value:"<operators>.assignmentExponentiation".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:92
let constant_44=builder.constant(Constant{name:"assignmentModulo".to_owned(),value:"<operators>.assignmentModulo".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:93
let constant_45=builder.constant(Constant{name:"assignmentShiftLeft".to_owned(),value:"<operators>.assignmentShiftLeft".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:94
let constant_46=builder.constant(Constant{name:"assignmentLogicalShiftRight".to_owned(),value:"<operators>.assignmentLogicalShiftRight".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:95
let constant_47=builder.constant(Constant{name:"assignmentArithmeticShiftRight".to_owned(),value:"<operators>.assignmentArithmeticShiftRight".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:96
let constant_48=builder.constant(Constant{name:"assignmentAnd".to_owned(),value:"<operators>.assignmentAnd".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:97
let constant_49=builder.constant(Constant{name:"assignmentOr".to_owned(),value:"<operators>.assignmentOr".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:98
let constant_50=builder.constant(Constant{name:"assignmentXor".to_owned(),value:"<operators>.assignmentXor".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:99
let constant_51=builder.constant(Constant{name:"arrayInitializer".to_owned(),value:"<operator>.arrayInitializer".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:100
let constant_52=builder.constant(Constant{name:"assignment".to_owned(),value:"<operator>.assignment".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:101
let constant_53=builder.constant(Constant{name:"minus".to_owned(),value:"<operator>.minus".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("E.g. `a = -b`".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:102
let constant_54=builder.constant(Constant{name:"plus".to_owned(),value:"<operator>.plus".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("E.g. `a = +b`".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:103
let constant_55=builder.constant(Constant{name:"preIncrement".to_owned(),value:"<operator>.preIncrement".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:104
let constant_56=builder.constant(Constant{name:"preDecrement".to_owned(),value:"<operator>.preDecrement".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:105
let constant_57=builder.constant(Constant{name:"postIncrement".to_owned(),value:"<operator>.postIncrement".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:106
let constant_58=builder.constant(Constant{name:"postDecrement".to_owned(),value:"<operator>.postDecrement".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:107
let constant_59=builder.constant(Constant{name:"logicalNot".to_owned(),value:"<operator>.logicalNot".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:108
let constant_60=builder.constant(Constant{name:"logicalOr".to_owned(),value:"<operator>.logicalOr".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:109
let constant_61=builder.constant(Constant{name:"logicalAnd".to_owned(),value:"<operator>.logicalAnd".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:110
let constant_62=builder.constant(Constant{name:"equals".to_owned(),value:"<operator>.equals".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:111
let constant_63=builder.constant(Constant{name:"notEquals".to_owned(),value:"<operator>.notEquals".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:112
let constant_64=builder.constant(Constant{name:"greaterThan".to_owned(),value:"<operator>.greaterThan".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:113
let constant_65=builder.constant(Constant{name:"lessThan".to_owned(),value:"<operator>.lessThan".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:114
let constant_66=builder.constant(Constant{name:"greaterEqualsThan".to_owned(),value:"<operator>.greaterEqualsThan".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:115
let constant_67=builder.constant(Constant{name:"lessEqualsThan".to_owned(),value:"<operator>.lessEqualsThan".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:116
let constant_68=builder.constant(Constant{name:"instanceOf".to_owned(),value:"<operator>.instanceOf".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:117
let constant_69=builder.constant(Constant{name:"memberAccess".to_owned(),value:"<operator>.memberAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deprecated. Formerly in C: `a.b` but not! in Java".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:118
let constant_70=builder.constant(Constant{name:"indirectMemberAccess".to_owned(),value:"<operator>.indirectMemberAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deprecated. Formerly in C: `a->b` and `a.b` in Java".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:119
let constant_71=builder.constant(Constant{name:"computedMemberAccess".to_owned(),value:"<operator>.computedMemberAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deprecated. Formerly in C: `a[b]` but not! in Java".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:120
let constant_72=builder.constant(Constant{name:"indirectComputedMemberAccess".to_owned(),value:"<operator>.indirectComputedMemberAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deprecated. Formerly in C++: `a->*b` and a[b] in Java".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:121
let constant_73=builder.constant(Constant{name:"indirection".to_owned(),value:"<operator>.indirection".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Accesses through a pointer (load/store), i.e. dereferences it. Typically the star-operator in C/C++. Pairs of addressOf and indirection operators cancel each other. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:122
let constant_74=builder.constant(Constant{name:"delete".to_owned(),value:"<operator>.delete".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Deletes a property from a namespace. E.g. `a=3; delete a; a == undefined;".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:123
let constant_75=builder.constant(Constant{name:"conditional".to_owned(),value:"<operator>.conditional".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("E.g. `a ? consequent : alternate`. In future probably also used for if statements".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:124
let constant_76=builder.constant(Constant{name:"elvis".to_owned(),value:"<operator>.elvis".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("E.g. `x = f() ?: g()`. A binary operator that returns its first operand if that operand evaluates to a true value, and otherwise evaluates and returns its second operand.".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:125
let constant_77=builder.constant(Constant{name:"cast".to_owned(),value:"<operator>.cast".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Type casts of any sort".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:126
let constant_78=builder.constant(Constant{name:"compare".to_owned(),value:"<operator>.compare".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Comparison between two arguments with the results: 0 == equal, negative == left < right, positive == left > right".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:127
let constant_79=builder.constant(Constant{name:"addressOf".to_owned(),value:"<operator>.addressOf".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns the address of a given object. Pairs of addressOf and indirection operators cancel each other. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:128
let constant_80=builder.constant(Constant{name:"sizeOf".to_owned(),value:"<operator>.sizeOf".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns the size of a given object".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:129
let constant_81=builder.constant(Constant{name:"fieldAccess".to_owned(),value:"<operator>.fieldAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns or sets a field/member of a struct/class. Typically, the dot-operator. In terms of CPG, the first argument is the struct/class and the second argument is either a FIELD_IDENTIFIER node, a LITERAL node or an arbitrary expression. If it is a FIELD_IDENTIFIER, then the string contained in the CANONICAL_NAME field of this FIELD_IDENTIFIER node selects which field/member is accessed. If it is a LITERAL node, then its CODE selects which field/member is selected. In any other case the access is considered unpredictable by the backend, and we overtaint. indexAccess and fieldAccess live in the same space, such that e.g. obj.field and obj[\"field\"] refer to the same target (as is correct in e.g. javascript). Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:130
let constant_82=builder.constant(Constant{name:"indirectFieldAccess".to_owned(),value:"<operator>.indirectFieldAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Accesses (loads/stores) to a field/member through a pointer to a struct/class. Typically, C/C++ arrow-operator. The field selection works the same way as for fieldAccess. For the sake of data-flow tracking, this is the same as first dereferencing the pointer and then accessing the field. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:131
let constant_83=builder.constant(Constant{name:"indexAccess".to_owned(),value:"<operator>.indexAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Accesses a container (e.g. array or associative array / dict / map) at an index. The index selection works the same way as for fieldAccess. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:132
let constant_84=builder.constant(Constant{name:"indirectIndexAccess".to_owned(),value:"<operator>.indirectIndexAccess".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Accesses an element of an Array through a pointer. The index selection works similar as for fieldAccess: However, the index must be an integer. If the second argument is a FIELD_ACCESS resp. LITERAL then its CANONICAL_NAME resp. CODE field is parsed as an integer; on parsing failure, the access is considered unpredictable and we overtaint. This is equivalent to a pointerShift followed by an indirection. This operator is currently only used by C-style languages. This behavior allows the backend to infer that ptr[0] and *ptr refer to the same object. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:133
let constant_85=builder.constant(Constant{name:"pointerShift".to_owned(),value:"<operator>.pointerShift".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Shifts a pointer. In terms of CPG, the first argument is the pointer and the second argument is the index. The index selection works the same way as for indirectIndexAccess. This operator is currently only used directly by the LLVM language, but it is also used internally for C. For example, pointerShift(ptr, 7) is equivalent to &(ptr[7]). Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:134
let constant_86=builder.constant(Constant{name:"getElementPtr".to_owned(),value:"<operator>.getElementPtr".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Derives a pointer to a field of a struct from a pointer to the entire struct. This corresponds to the C idiom &(ptr->field), which does not access memory. The field selection works the same way as for fieldAccess. This operator is currently only used directly by the LLVM language. Handling of this operator is special-cased in the back-end".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:135
let constant_87=builder.constant(Constant{name:"formatString".to_owned(),value:"<operator>.formatString".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Defines a string literal which can replace containing expressions with their values.".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:136
let constant_88=builder.constant(Constant{name:"formattedValue".to_owned(),value:"<operator>.formattedValue".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Replaces an expression with its string value.".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:137
let constant_89=builder.constant(Constant{name:"range".to_owned(),value:"<operator>.range".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Defines a range of values, e.g. `for (i in 1..10) print(i)`.".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:138
let constant_90=builder.constant(Constant{name:"in".to_owned(),value:"<operator>.in".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Checks the existence of a variable in a range or collection, e.g. `for (i in 1..10) print(i)`.".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:139
let constant_91=builder.constant(Constant{name:"notIn".to_owned(),value:"<operator>.notIn".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Checks the non-existence of a variable in a range or collection, e.g. `print(5 not in [1, 2, 3, 4])`".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:140
let constant_92=builder.constant(Constant{name:"is".to_owned(),value:"<operator>.is".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Checks whether a variable is of a given type".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:141
let constant_93=builder.constant(Constant{name:"isNot".to_owned(),value:"<operator>.isNot".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Checks whether a variable is not of a given type".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:142
let constant_94=builder.constant(Constant{name:"notNullAssert".to_owned(),value:"<operator>.notNullAssert".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Converts any value to a not-null type".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:143
let constant_95=builder.constant(Constant{name:"lengthOf".to_owned(),value:"<operator>.lengthOf".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns the length of the given collection e.g. (new int[]{ 1, 2, 3 }).length in Java or len([1, 2, 3]) in Python".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:144
let constant_96=builder.constant(Constant{name:"safeNavigation".to_owned(),value:"<operator>.safeNavigation".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns null if the first operator is null, otherwise performs a dereferencing operation".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:145
let constant_97=builder.constant(Constant{name:"tryCatch".to_owned(),value:"<operator>.tryCatch".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Returns either the last expression in the `try`-block, or the last expression in one of the `catch`/`except`-blocks. The contents of the `finally`-block are ignored.".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:146
let constant_98=builder.constant(Constant{name:"alloc".to_owned(),value:"<operator>.alloc".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Allocates memory for an object of a specific type".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:147
let constant_99=builder.constant(Constant{name:"importCall".to_owned(),value:"<operator>.import".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Imports a package".to_owned()),schema_info:Some(info_3.clone())});
// CPG_CONSTRUCTION_OP:148
builder.add_constants("Operators".to_owned(),[constant_26.clone(),constant_27.clone(),constant_28.clone(),constant_29.clone(),constant_30.clone(),constant_31.clone(),constant_32.clone(),constant_33.clone(),constant_34.clone(),constant_35.clone(),constant_36.clone(),constant_37.clone(),constant_38.clone(),constant_39.clone(),constant_40.clone(),constant_41.clone(),constant_42.clone(),constant_43.clone(),constant_44.clone(),constant_45.clone(),constant_46.clone(),constant_47.clone(),constant_48.clone(),constant_49.clone(),constant_50.clone(),constant_51.clone(),constant_52.clone(),constant_53.clone(),constant_54.clone(),constant_55.clone(),constant_56.clone(),constant_57.clone(),constant_58.clone(),constant_59.clone(),constant_60.clone(),constant_61.clone(),constant_62.clone(),constant_63.clone(),constant_64.clone(),constant_65.clone(),constant_66.clone(),constant_67.clone(),constant_68.clone(),constant_69.clone(),constant_70.clone(),constant_71.clone(),constant_72.clone(),constant_73.clone(),constant_74.clone(),constant_75.clone(),constant_76.clone(),constant_77.clone(),constant_78.clone(),constant_79.clone(),constant_80.clone(),constant_81.clone(),constant_82.clone(),constant_83.clone(),constant_84.clone(),constant_85.clone(),constant_86.clone(),constant_87.clone(),constant_88.clone(),constant_89.clone(),constant_90.clone(),constant_91.clone(),constant_92.clone(),constant_93.clone(),constant_94.clone(),constant_95.clone(),constant_96.clone(),constant_97.clone(),constant_98.clone(),constant_99.clone(),]);
 Ok(Self{operators:vec![constant_26.clone(),constant_27.clone(),constant_28.clone(),constant_29.clone(),constant_30.clone(),constant_31.clone(),constant_32.clone(),constant_33.clone(),constant_34.clone(),constant_35.clone(),constant_36.clone(),constant_37.clone(),constant_38.clone(),constant_39.clone(),constant_40.clone(),constant_41.clone(),constant_42.clone(),constant_43.clone(),constant_44.clone(),constant_45.clone(),constant_46.clone(),constant_47.clone(),constant_48.clone(),constant_49.clone(),constant_50.clone(),constant_51.clone(),constant_52.clone(),constant_53.clone(),constant_54.clone(),constant_55.clone(),constant_56.clone(),constant_57.clone(),constant_58.clone(),constant_59.clone(),constant_60.clone(),constant_61.clone(),constant_62.clone(),constant_63.clone(),constant_64.clone(),constant_65.clone(),constant_66.clone(),constant_67.clone(),constant_68.clone(),constant_69.clone(),constant_70.clone(),constant_71.clone(),constant_72.clone(),constant_73.clone(),constant_74.clone(),constant_75.clone(),constant_76.clone(),constant_77.clone(),constant_78.clone(),constant_79.clone(),constant_80.clone(),constant_81.clone(),constant_82.clone(),constant_83.clone(),constant_84.clone(),constant_85.clone(),constant_86.clone(),constant_87.clone(),constant_88.clone(),constant_89.clone(),constant_90.clone(),constant_91.clone(),constant_92.clone(),constant_93.clone(),constant_94.clone(),constant_95.clone(),constant_96.clone(),constant_97.clone(),constant_98.clone(),constant_99.clone(),],})} pub fn apply(builder:&mut ObjectSchemaBuilder,)->Result<Self>{Self::new(builder,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Operators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Operators".to_owned(),schema_name:"Operators".to_owned(), doc_index:17,description:"\n\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for OperatorsSchema { fn doc_index(&self)->i32{17} fn description(&self)->&str{"\n\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Operators
// CPG_SCHEMA_MODULE:Configuration
#[derive(Clone,Debug)] pub struct ConfigurationSchema {pub config_file:NodeRef,}impl ConfigurationSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,)->Result<Self>{let _object_10=_arg_base.content.clone();let _object_6=_arg_base.name.clone();let info_4=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Configuration$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Configuration".to_owned(),schema_name:"Configuration".to_owned(), doc_index:18,description:"\nThe code property graph specification currently does not contain\nschema elements for the representation of configuration files in\na structured format, however, it does allow configuration files\nto be included verbatim in the graph to enable language-/framework-\nspecific passes to access them. This layer provides the necessary\nschema elements for this basic support of configuration files.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:149
let _object_100=builder.add_node_type("CONFIG_FILE".to_owned(),"This node type represent a configuration file, where `NAME` is the name\nof the file and `content` is its content. The exact representation of the\nname is left undefined and can be chosen as required by consumers of\nthe corresponding configuration files.",Some(info_4.clone()));
// CPG_CONSTRUCTION_OP:150
_object_100.proto_id(50);
// CPG_CONSTRUCTION_OP:151
_object_100.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:152
_object_100.add_property(&_object_10);
 Ok(Self{config_file:_object_100,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,)->Result<Self>{Self::new(builder,_arg_base,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Configuration$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Configuration".to_owned(),schema_name:"Configuration".to_owned(), doc_index:18,description:"\nThe code property graph specification currently does not contain\nschema elements for the representation of configuration files in\na structured format, however, it does allow configuration files\nto be included verbatim in the graph to enable language-/framework-\nspecific passes to access them. This layer provides the necessary\nschema elements for this basic support of configuration files.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for ConfigurationSchema { fn doc_index(&self)->i32{18} fn description(&self)->&str{"\nThe code property graph specification currently does not contain\nschema elements for the representation of configuration files in\na structured format, however, it does allow configuration files\nto be included verbatim in the graph to enable language-/framework-\nspecific passes to access them. This layer provides the necessary\nschema elements for this basic support of configuration files.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Configuration
// CPG_SCHEMA_MODULE:MetaData
#[derive(Clone,Debug)] pub struct MetaDataSchema {pub overlays:PropertyRef,pub language:PropertyRef,pub root:PropertyRef,pub meta_data:NodeRef,pub languages:Vec<ConstantRef>,}impl MetaDataSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,)->Result<Self>{let _object_1=_arg_base.version.clone();let _object_2=_arg_base.hash.clone();let info_5=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.MetaData$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.MetaData".to_owned(),schema_name:"MetaData".to_owned(), doc_index:2,description:"\nThe Meta Data Layer contains information about CPG creation. In particular,\nit indicates which language frontend generated the CPG and which overlays\nhave been applied. The layer consists of a single node - the Meta Data node -\nand language frontends MUST create this node. Overlay creators MUST edit\nthis node to indicate that a layer has been successfully applied in all\ncases where applying the layer more than once is prohibitive.\n".to_owned(),provided_by_frontend:true};// CPG_CONSTRUCTION_OP:153
let _object_101=builder.add_property("OVERLAYS".to_owned(),ValueType::String,"The field contains the names of the overlays applied to this CPG, in order of their\napplication. Names are free-form strings, that is, this specification does not\ndictate them but rather requires tool producers and consumers to communicate them\nbetween each other.",Some(info_5.clone()));
// CPG_CONSTRUCTION_OP:154
_object_101.as_list();
// CPG_CONSTRUCTION_OP:155
_object_101.proto_id(118);
// CPG_CONSTRUCTION_OP:156
let _object_102=builder.add_property("LANGUAGE".to_owned(),ValueType::String,"This field indicates which CPG language frontend generated the CPG.\nFrontend developers may freely choose a value that describes their frontend\nso long as it is not used by an existing frontend. Reserved values are to date:\nC, LLVM, GHIDRA, PHP.",Some(info_5.clone()));
// CPG_CONSTRUCTION_OP:157
_object_102.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:158
_object_102.proto_id(19);
// CPG_CONSTRUCTION_OP:159
let _object_103=builder.add_property("ROOT".to_owned(),ValueType::String,"The path to the root directory of the source/binary this CPG is generated from.",Some(info_5.clone()));
// CPG_CONSTRUCTION_OP:160
_object_103.proto_id(1199);
// CPG_CONSTRUCTION_OP:161
_object_103.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:162
let _object_104=builder.add_node_type("META_DATA".to_owned(),"This node contains the CPG meta data. Exactly one node of this type\nMUST exist per CPG. The `HASH` property MAY contain a hash value calculated\nover the source files this CPG was generated from. The `VERSION` MUST be\nset to the version of the specification (\"1.1\"). The language field indicates\nwhich language frontend was used to generate the CPG and the list property\n`OVERLAYS` specifies which overlays have been applied to the CPG.",Some(info_5.clone()));
// CPG_CONSTRUCTION_OP:163
_object_104.proto_id(39);
// CPG_CONSTRUCTION_OP:164
_object_104.add_property(&_object_102);
// CPG_CONSTRUCTION_OP:165
_object_104.add_property(&_object_1);
// CPG_CONSTRUCTION_OP:166
_object_104.add_property(&_object_101);
// CPG_CONSTRUCTION_OP:167
_object_104.add_property(&_object_2);
// CPG_CONSTRUCTION_OP:168
_object_104.add_property(&_object_103);
// CPG_CONSTRUCTION_OP:169
let constant_105=builder.constant(Constant{name:"JAVA".to_owned(),value:"JAVA".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:170
constant_105.proto_id(1);
// CPG_CONSTRUCTION_OP:171
let constant_106=builder.constant(Constant{name:"JAVASCRIPT".to_owned(),value:"JAVASCRIPT".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:172
constant_106.proto_id(2);
// CPG_CONSTRUCTION_OP:173
let constant_107=builder.constant(Constant{name:"GOLANG".to_owned(),value:"GOLANG".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:174
constant_107.proto_id(3);
// CPG_CONSTRUCTION_OP:175
let constant_108=builder.constant(Constant{name:"CSHARP".to_owned(),value:"CSHARP".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:176
constant_108.proto_id(4);
// CPG_CONSTRUCTION_OP:177
let constant_109=builder.constant(Constant{name:"ABAP".to_owned(),value:"ABAP".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:178
constant_109.proto_id(20);
// CPG_CONSTRUCTION_OP:179
let constant_110=builder.constant(Constant{name:"C".to_owned(),value:"C".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:180
constant_110.proto_id(5);
// CPG_CONSTRUCTION_OP:181
let constant_111=builder.constant(Constant{name:"PYTHON".to_owned(),value:"PYTHON".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:182
constant_111.proto_id(6);
// CPG_CONSTRUCTION_OP:183
let constant_112=builder.constant(Constant{name:"LLVM".to_owned(),value:"LLVM".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:184
constant_112.proto_id(7);
// CPG_CONSTRUCTION_OP:185
let constant_113=builder.constant(Constant{name:"PHP".to_owned(),value:"PHP".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:186
constant_113.proto_id(8);
// CPG_CONSTRUCTION_OP:187
let constant_114=builder.constant(Constant{name:"FUZZY_TEST_LANG".to_owned(),value:"FUZZY_TEST_LANG".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:188
constant_114.proto_id(9);
// CPG_CONSTRUCTION_OP:189
let constant_115=builder.constant(Constant{name:"GHIDRA".to_owned(),value:"GHIDRA".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("generic reverse engineering framework".to_owned()),schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:190
constant_115.proto_id(10);
// CPG_CONSTRUCTION_OP:191
let constant_116=builder.constant(Constant{name:"KOTLIN".to_owned(),value:"KOTLIN".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:192
constant_116.proto_id(11);
// CPG_CONSTRUCTION_OP:193
let constant_117=builder.constant(Constant{name:"NEWC".to_owned(),value:"NEWC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Eclipse CDT based parser for C/C++".to_owned()),schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:194
constant_117.proto_id(12);
// CPG_CONSTRUCTION_OP:195
let constant_118=builder.constant(Constant{name:"JAVASRC".to_owned(),value:"JAVASRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based front-end for Java".to_owned()),schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:196
constant_118.proto_id(13);
// CPG_CONSTRUCTION_OP:197
let constant_119=builder.constant(Constant{name:"PYTHONSRC".to_owned(),value:"PYTHONSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based front-end for Python".to_owned()),schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:198
constant_119.proto_id(14);
// CPG_CONSTRUCTION_OP:199
let constant_120=builder.constant(Constant{name:"JSSRC".to_owned(),value:"JSSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based JS frontend based on Babel".to_owned()),schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:200
constant_120.proto_id(15);
// CPG_CONSTRUCTION_OP:201
let constant_121=builder.constant(Constant{name:"RUBYSRC".to_owned(),value:"RUBYSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based frontend for Ruby".to_owned()),schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:202
constant_121.proto_id(17);
// CPG_CONSTRUCTION_OP:203
let constant_122=builder.constant(Constant{name:"SWIFTSRC".to_owned(),value:"SWIFTSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based frontend for Swift".to_owned()),schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:204
constant_122.proto_id(18);
// CPG_CONSTRUCTION_OP:205
let constant_123=builder.constant(Constant{name:"CSHARPSRC".to_owned(),value:"CSHARPSRC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Source-based frontend for C# and .NET".to_owned()),schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:206
constant_123.proto_id(19);
// CPG_CONSTRUCTION_OP:207
let constant_124=builder.constant(Constant{name:"RUST".to_owned(),value:"RUST".to_owned(),value_type:ValueType::String,proto_id:None,comment:None,schema_info:Some(info_5.clone())});
// CPG_CONSTRUCTION_OP:208
constant_124.proto_id(21);
// CPG_CONSTRUCTION_OP:209
builder.add_constants("Languages".to_owned(),[constant_105.clone(),constant_106.clone(),constant_107.clone(),constant_108.clone(),constant_109.clone(),constant_110.clone(),constant_111.clone(),constant_112.clone(),constant_113.clone(),constant_114.clone(),constant_115.clone(),constant_116.clone(),constant_117.clone(),constant_118.clone(),constant_119.clone(),constant_120.clone(),constant_121.clone(),constant_122.clone(),constant_123.clone(),constant_124.clone(),]);
 Ok(Self{overlays:_object_101,language:_object_102,root:_object_103,meta_data:_object_104,languages:vec![constant_105.clone(),constant_106.clone(),constant_107.clone(),constant_108.clone(),constant_109.clone(),constant_110.clone(),constant_111.clone(),constant_112.clone(),constant_113.clone(),constant_114.clone(),constant_115.clone(),constant_116.clone(),constant_117.clone(),constant_118.clone(),constant_119.clone(),constant_120.clone(),constant_121.clone(),constant_122.clone(),constant_123.clone(),constant_124.clone(),],})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,)->Result<Self>{Self::new(builder,_arg_base,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.MetaData$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.MetaData".to_owned(),schema_name:"MetaData".to_owned(), doc_index:2,description:"\nThe Meta Data Layer contains information about CPG creation. In particular,\nit indicates which language frontend generated the CPG and which overlays\nhave been applied. The layer consists of a single node - the Meta Data node -\nand language frontends MUST create this node. Overlay creators MUST edit\nthis node to indicate that a layer has been successfully applied in all\ncases where applying the layer more than once is prohibitive.\n".to_owned(),provided_by_frontend:true}} } impl SchemaBase for MetaDataSchema { fn doc_index(&self)->i32{2} fn description(&self)->&str{"\nThe Meta Data Layer contains information about CPG creation. In particular,\nit indicates which language frontend generated the CPG and which overlays\nhave been applied. The layer consists of a single node - the Meta Data node -\nand language frontends MUST create this node. Overlay creators MUST edit\nthis node to indicate that a layer has been successfully applied in all\ncases where applying the layer more than once is prohibitive.\n"} fn provided_by_frontend(&self)->bool{true} }// CPG_SCHEMA_MODULE_END:MetaData
// CPG_SCHEMA_MODULE:Type
#[derive(Clone,Debug)] pub struct TypeSchema {pub type_full_name:PropertyRef,pub alias_type_full_name:PropertyRef,pub inherits_from_type_full_name:PropertyRef,pub type_decl_full_name:PropertyRef,pub type_decl:NodeRef,pub type_parameter:NodeRef,pub type_argument:NodeRef,pub member:NodeRef,pub tpe:NodeRef,pub binds_to:EdgeRef,pub alias_of:EdgeRef,pub inherits_from:EdgeRef,}impl TypeSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{let _object_11=_arg_base.ast_parent_type.clone();let _object_12=_arg_base.ast_parent_full_name.clone();let _object_13=_arg_base.declaration.clone();let _object_14=_arg_base.r#ref.clone();let _object_15=_arg_fs.filename.clone();let _object_20=_arg_fs.source_file.clone();let _object_21=_arg_fs.file.clone();let _object_4=_arg_base.is_external.clone();let _object_6=_arg_base.name.clone();let _object_7=_arg_base.full_name.clone();let info_6=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Type$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Type".to_owned(),schema_name:"Type".to_owned(), doc_index:6,description:"\nThe Type Layer contains information about type declarations, relations between\ntypes, and type instantiation and usage. In its current form, it allows modelling of\nparametrized types, type hierarchies and aliases.\n".to_owned(),provided_by_frontend:true};// CPG_CONSTRUCTION_OP:210
let _object_125=builder.add_property("TYPE_FULL_NAME".to_owned(),ValueType::String,"This field contains the fully-qualified static type name of the program\nconstruct represented by a node. It is the name of an instantiated type, e.g.,\n`java.util.List<Integer>`, rather than `java.util.List[T]`. If the type\ncannot be determined, this field should be set to the empty string.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:211
_object_125.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:212
_object_125.proto_id(51);
// CPG_CONSTRUCTION_OP:213
let _object_126=builder.add_property("ALIAS_TYPE_FULL_NAME".to_owned(),ValueType::String,"This property holds the fully qualified name of the type that the node is\na type alias of.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:214
_object_126.proto_id(158);
// CPG_CONSTRUCTION_OP:215
let _object_127=builder.add_property("INHERITS_FROM_TYPE_FULL_NAME".to_owned(),ValueType::String,"The static types a TYPE_DECL inherits from. This property is matched against the\nFULL_NAME of TYPE nodes and thus it is required to have at least one TYPE node\nfor each TYPE_FULL_NAME",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:216
_object_127.as_list();
// CPG_CONSTRUCTION_OP:217
_object_127.proto_id(53);
// CPG_CONSTRUCTION_OP:218
let _object_128=builder.add_property("TYPE_DECL_FULL_NAME".to_owned(),ValueType::String,"The static type decl of a TYPE. This property is matched against the FULL_NAME\nof TYPE_DECL nodes. It is required to have exactly one TYPE_DECL for each\ndifferent TYPE_DECL_FULL_NAME",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:219
_object_128.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:220
_object_128.proto_id(52);
// CPG_CONSTRUCTION_OP:221
let _object_129=builder.add_node_type("TYPE_DECL".to_owned(),"This node represents a type declaration as for example given by a class-, struct-,\nor union declaration. In contrast to a `TYPE` node, this node does not represent a\nconcrete instantiation of a type, e.g., for the parametrized type `List[T]`, it represents\n`List[T]`, but not `List[Integer]` where `Integer` is a concrete type.\n\nThe language frontend MUST create type declarations for all types declared in the\nsource program and MAY provide type declarations for types that are not declared\nbut referenced by the source program. If a declaration is present in the source\nprogram, the field `IS_EXTERNAL` is set to `false`. Otherwise, it is set to `true`.\n\nThe `FULL_NAME` field specifies the type's fully-qualified name, including\ninformation about the namespace it is contained in if applicable, the name field\nis the type's short name. Line and column number information is specified in the\noptional fields `LINE_NUMBER`, `COLUMN_NUMBER`, `LINE_NUMBER_END`, and\n`COLUMN_NUMBER_END` and the name of the source file is specified in `FILENAME`.\n\nBase types can be specified via the `INHERITS_FROM_TYPE_FULL_NAME` list, where\neach entry contains the fully-qualified name of a base type. If the type is\nknown to be an alias of another type (as for example introduced via the C\n`typedef` statement), the name of the alias is stored in `ALIAS_TYPE_FULL_NAME`.\n\nFinally, the fully qualified name of the program constructs that the type declaration\nis immediately contained in is stored in the `AST_PARENT_FULL_NAME` field\nand its type is indicated in the `AST_PARENT_TYPE` field to be one of\n`METHOD`, `TYPE_DECL` or `NAMESPACE_BLOCK`.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:222
_object_129.proto_id(46);
// CPG_CONSTRUCTION_OP:223
_object_129.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:224
_object_129.add_property(&_object_7);
// CPG_CONSTRUCTION_OP:225
_object_129.add_property(&_object_4);
// CPG_CONSTRUCTION_OP:226
_object_129.add_property(&_object_127);
// CPG_CONSTRUCTION_OP:227
_object_129.add_property(&_object_126);
// CPG_CONSTRUCTION_OP:228
_object_129.add_property(&_object_15);
// CPG_CONSTRUCTION_OP:229
_object_129.add_property(&_object_11);
// CPG_CONSTRUCTION_OP:230
_object_129.add_property(&_object_12);
// CPG_CONSTRUCTION_OP:231
_object_129.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:232
let _object_130=builder.add_node_type("TYPE_PARAMETER".to_owned(),"This node represents a formal type parameter, that is, the type parameter\nas given in a type-parametrized method or type declaration. Examples for\nlanguages that support type parameters are Java (via Generics) and C++\n(via templates). Apart from the standard fields of AST nodes, the type\nparameter carries only a `NAME` field that holds the parameters name.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:233
_object_130.proto_id(47);
// CPG_CONSTRUCTION_OP:234
_object_130.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:235
let _object_131=builder.add_node_type("TYPE_ARGUMENT".to_owned(),"An (actual) type argument as used to instantiate a parametrized type, in the\nsame way an (actual) arguments provides concrete values for a parameter\nat method call sites. As it true for arguments, the method is not expected\nto  interpret the type argument. It MUST however store its code in the\n`CODE` field.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:236
_object_131.proto_id(48);
// CPG_CONSTRUCTION_OP:237
let _object_132=builder.add_node_type("MEMBER".to_owned(),"This node represents a type member of a class, struct or union, e.g., for the\n type declaration `class Foo{ int i ; }`, it represents the declaration of the\n variable `i`.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:238
_object_132.proto_id(9);
// CPG_CONSTRUCTION_OP:239
_object_132.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:240
_object_132.add_property(&_object_11);
// CPG_CONSTRUCTION_OP:241
_object_132.add_property(&_object_12);
// CPG_CONSTRUCTION_OP:242
_object_132.extend(&_object_13)?;
// CPG_CONSTRUCTION_OP:243
_object_132.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:244
let _object_133=builder.add_node_type("TYPE".to_owned(),"This node represents a type instance, that is, a concrete instantiation\nof a type declaration.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:245
_object_133.proto_id(45);
// CPG_CONSTRUCTION_OP:246
_object_133.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:247
_object_133.add_property(&_object_7);
// CPG_CONSTRUCTION_OP:248
_object_133.add_property(&_object_128);
// CPG_CONSTRUCTION_OP:249
_object_133.starter_name(Some("typ".to_owned()));
// CPG_CONSTRUCTION_OP:250
_object_133.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:251
let _object_134=builder.add_edge_type("BINDS_TO".to_owned(),"This edge connects type arguments to type parameters to indicate\nthat the type argument is used to instantiate the type parameter.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:252
_object_134.proto_id(22);
// CPG_CONSTRUCTION_OP:253
let _object_135=builder.add_edge_type("ALIAS_OF".to_owned(),"This edge represents an alias relation between a type declaration and a type.\nThe language frontend MUST NOT create `ALIAS_OF` edges as they are created\nautomatically based on `ALIAS_TYPE_FULL_NAME` fields when the CPG is first loaded.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:254
_object_135.proto_id(138);
// CPG_CONSTRUCTION_OP:255
let _object_136=builder.add_edge_type("INHERITS_FROM".to_owned(),"Inheritance relation between a type declaration and a type. This edge MUST NOT\n be created by the language frontend as it is automatically created from\n `INHERITS_FROM_TYPE_FULL_NAME` fields then the CPG is first loaded.",Some(info_6.clone()));
// CPG_CONSTRUCTION_OP:256
_object_136.proto_id(23);
// CPG_CONSTRUCTION_OP:257
_object_129.add_out_edge(&_object_136,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:258
_object_129.add_out_edge(&_object_135,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"aliasedType".to_owned(),out_doc:"".to_owned(),in_step:"aliasTypeDecl".to_owned(),in_doc:"Direct alias type declarations.".to_owned()});
// CPG_CONSTRUCTION_OP:259
_object_129.add_out_edge(&_object_20,&_object_21,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"typeDecl".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:260
_object_131.add_out_edge(&_object_134,&_object_130,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:261
_object_131.add_out_edge(&_object_14,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{type_full_name:_object_125,alias_type_full_name:_object_126,inherits_from_type_full_name:_object_127,type_decl_full_name:_object_128,type_decl:_object_129,type_parameter:_object_130,type_argument:_object_131,member:_object_132,tpe:_object_133,binds_to:_object_134,alias_of:_object_135,inherits_from:_object_136,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{Self::new(builder,_arg_base,_arg_fs,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Type$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Type".to_owned(),schema_name:"Type".to_owned(), doc_index:6,description:"\nThe Type Layer contains information about type declarations, relations between\ntypes, and type instantiation and usage. In its current form, it allows modelling of\nparametrized types, type hierarchies and aliases.\n".to_owned(),provided_by_frontend:true}} } impl SchemaBase for TypeSchema { fn doc_index(&self)->i32{6} fn description(&self)->&str{"\nThe Type Layer contains information about type declarations, relations between\ntypes, and type instantiation and usage. In its current form, it allows modelling of\nparametrized types, type hierarchies and aliases.\n"} fn provided_by_frontend(&self)->bool{true} }// CPG_SCHEMA_MODULE_END:Type
// CPG_SCHEMA_MODULE:Method
#[derive(Clone,Debug)] pub struct MethodSchema {pub signature:PropertyRef,pub method:NodeRef,pub is_variadic:PropertyRef,pub method_parameter_in:NodeRef,pub method_parameter_out:NodeRef,pub method_return:NodeRef,}impl MethodSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_type_decl_schema:&TypeSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{let _object_11=_arg_base.ast_parent_type.clone();let _object_12=_arg_base.ast_parent_full_name.clone();let _object_125=_arg_type_decl_schema.type_full_name.clone();let _object_13=_arg_base.declaration.clone();let _object_15=_arg_fs.filename.clone();let _object_18=_arg_fs.line_number_end.clone();let _object_19=_arg_fs.column_number_end.clone();let _object_2=_arg_base.hash.clone();let _object_20=_arg_fs.source_file.clone();let _object_21=_arg_fs.file.clone();let _object_4=_arg_base.is_external.clone();let _object_5=_arg_base.index.clone();let _object_6=_arg_base.name.clone();let _object_7=_arg_base.full_name.clone();let info_7=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Method$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Method".to_owned(),schema_name:"Method".to_owned(), doc_index:5,description:"\nThe Method Layer contains declarations of methods, functions, and procedures.\nInput parameters and output parameters (including return parameters) are\nrepresented, however, method contents is not present in this layer.\n".to_owned(),provided_by_frontend:true};// CPG_CONSTRUCTION_OP:262
let _object_137=builder.add_property("SIGNATURE".to_owned(),ValueType::String,"The method signature encodes the types of parameters in a string.\nThe string SHOULD be human readable and suitable for differentiating methods\nwith different parameter types sufficiently to allow for resolving of\nfunction overloading. The present specification does not enforce a strict\nformat for the signature, that is, it can be chosen by the frontend\nimplementor to fit the source language.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:263
_object_137.mandatory(Scalar::String { value:"".to_owned() })?;
// CPG_CONSTRUCTION_OP:264
_object_137.proto_id(22);
// CPG_CONSTRUCTION_OP:265
let _object_138=builder.add_node_type("METHOD".to_owned(),"Programming languages offer many closely-related concepts for describing blocks\nof code that can be executed with input parameters and return output parameters,\npossibly causing side effects. In the CPG specification, we refer to all of these\nconcepts (procedures, functions, methods, etc.) as methods. A single METHOD node\nmust exist for each method found in the source program.\n\nThe `FULL_NAME` field specifies the method's fully-qualified name, including\ninformation about the namespace it is contained in if applicable, the name field\nis the function's short name. The field `IS_EXTERNAL` indicates whether it was\npossible to identify a method body for the method. This is true for methods that\nare defined in the source program, and false for methods that are dynamically\nlinked to the program, that is, methods that exist in an external dependency.\n\nLine and column number information is specified in the optional fields\n`LINE_NUMBER`, `COLUMN_NUMBER`, `LINE_NUMBER_END`, and `COLUMN_NUMBER_END` and\nthe name of the source file is specified in `FILENAME`. An optional hash value\nMAY be calculated over the function contents and included in the `HASH` field.\n\nFinally, the fully qualified name of the program constructs that the method\nis immediately contained in is stored in the `AST_PARENT_FULL_NAME` field\nand its type is indicated in the `AST_PARENT_TYPE` field to be one of\n`METHOD`, `TYPE_DECL` or `NAMESPACE_BLOCK`.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:266
_object_138.proto_id(1);
// CPG_CONSTRUCTION_OP:267
_object_138.add_property(&_object_7);
// CPG_CONSTRUCTION_OP:268
_object_138.add_property(&_object_4);
// CPG_CONSTRUCTION_OP:269
_object_138.add_property(&_object_137);
// CPG_CONSTRUCTION_OP:270
_object_138.add_property(&_object_18);
// CPG_CONSTRUCTION_OP:271
_object_138.add_property(&_object_19);
// CPG_CONSTRUCTION_OP:272
_object_138.add_property(&_object_15);
// CPG_CONSTRUCTION_OP:273
_object_138.add_property(&_object_2);
// CPG_CONSTRUCTION_OP:274
_object_138.add_property(&_object_11);
// CPG_CONSTRUCTION_OP:275
_object_138.add_property(&_object_12);
// CPG_CONSTRUCTION_OP:276
_object_138.extend(&_object_13)?;
// CPG_CONSTRUCTION_OP:277
_object_138.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:278
let _object_139=builder.add_property("IS_VARIADIC".to_owned(),ValueType::Boolean,"Specifies whether a parameter is the variadic argument handling parameter of\na variadic method. Only one parameter of a method is allowed to have this\nproperty set to true.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:279
_object_139.mandatory(Scalar::Boolean { value:false })?;
// CPG_CONSTRUCTION_OP:280
_object_139.proto_id(221);
// CPG_CONSTRUCTION_OP:281
let _object_140=builder.add_node_type("METHOD_PARAMETER_IN".to_owned(),"This node represents a formal input parameter. The field `NAME` contains its\nname, while the field `TYPE_FULL_NAME` contains the fully qualified type name.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:282
_object_140.proto_id(34);
// CPG_CONSTRUCTION_OP:283
_object_140.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:284
_object_140.add_property(&_object_139);
// CPG_CONSTRUCTION_OP:285
_object_140.add_property(&_object_5);
// CPG_CONSTRUCTION_OP:286
_object_140.extend(&_object_13)?;
// CPG_CONSTRUCTION_OP:287
_object_140.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:288
let _object_141=builder.add_node_type("METHOD_PARAMETER_OUT".to_owned(),"This node represents a formal output parameter. Corresponding output parameters\nfor input parameters MUST NOT be created by the frontend as they are automatically\ncreated upon first loading the CPG.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:289
_object_141.proto_id(33);
// CPG_CONSTRUCTION_OP:290
_object_141.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:291
_object_141.add_property(&_object_139);
// CPG_CONSTRUCTION_OP:292
_object_141.add_property(&_object_5);
// CPG_CONSTRUCTION_OP:293
_object_141.extend(&_object_13)?;
// CPG_CONSTRUCTION_OP:294
let _object_142=builder.add_node_type("METHOD_RETURN".to_owned(),"This node represents an (unnamed) formal method return parameter. It carries its\nfully qualified type name in `TYPE_FULL_NAME`. The `CODE` field MAY be set freely,\ne.g., to the constant `RET`, however, subsequent layer creators MUST NOT depend\non this value.",Some(info_7.clone()));
// CPG_CONSTRUCTION_OP:295
_object_142.proto_id(3);
// CPG_CONSTRUCTION_OP:296
_object_142.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:297
_object_138.add_out_edge(&_object_20,&_object_21,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"method".to_owned(),in_doc:"".to_owned()});
 Ok(Self{signature:_object_137,method:_object_138,is_variadic:_object_139,method_parameter_in:_object_140,method_parameter_out:_object_141,method_return:_object_142,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_type_decl_schema:&TypeSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{Self::new(builder,_arg_base,_arg_type_decl_schema,_arg_fs,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Method$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Method".to_owned(),schema_name:"Method".to_owned(), doc_index:5,description:"\nThe Method Layer contains declarations of methods, functions, and procedures.\nInput parameters and output parameters (including return parameters) are\nrepresented, however, method contents is not present in this layer.\n".to_owned(),provided_by_frontend:true}} } impl SchemaBase for MethodSchema { fn doc_index(&self)->i32{5} fn description(&self)->&str{"\nThe Method Layer contains declarations of methods, functions, and procedures.\nInput parameters and output parameters (including return parameters) are\nrepresented, however, method contents is not present in this layer.\n"} fn provided_by_frontend(&self)->bool{true} }// CPG_SCHEMA_MODULE_END:Method
// CPG_SCHEMA_MODULE:Ast
#[derive(Clone,Debug)] pub struct AstSchema {pub order:PropertyRef,pub ast_node:NodeRef,pub block:NodeRef,pub literal:NodeRef,pub local:NodeRef,pub identifier:NodeRef,pub canonical_name:PropertyRef,pub field_identifier:NodeRef,pub modifier_type:PropertyRef,pub modifier_types:Vec<ConstantRef>,pub modifier:NodeRef,pub jump_target:NodeRef,pub jump_label:NodeRef,pub method_ref:NodeRef,pub type_ref:NodeRef,pub ret:NodeRef,pub control_structure_type:PropertyRef,pub control_structure_types:Vec<ConstantRef>,pub control_structure:NodeRef,pub unknown:NodeRef,pub ast:EdgeRef,pub condition:EdgeRef,pub true_body:EdgeRef,pub false_body:EdgeRef,pub do_body:EdgeRef,pub for_init:EdgeRef,pub for_update:EdgeRef,pub for_body:EdgeRef,pub try_body:EdgeRef,pub catch_body:EdgeRef,pub finally_body:EdgeRef,pub jump_argument:EdgeRef,pub call_repr:NodeRef,pub call_node:NodeRef,pub expression:NodeRef,}impl AstSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_namespaces:&NamespaceSchema,_arg_method_schema:&MethodSchema,_arg_type_schema:&TypeSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{let _object_125=_arg_type_schema.type_full_name.clone();let _object_129=_arg_type_schema.type_decl.clone();let _object_13=_arg_base.declaration.clone();let _object_130=_arg_type_schema.type_parameter.clone();let _object_131=_arg_type_schema.type_argument.clone();let _object_132=_arg_type_schema.member.clone();let _object_133=_arg_type_schema.tpe.clone();let _object_137=_arg_method_schema.signature.clone();let _object_138=_arg_method_schema.method.clone();let _object_14=_arg_base.r#ref.clone();let _object_140=_arg_method_schema.method_parameter_in.clone();let _object_141=_arg_method_schema.method_parameter_out.clone();let _object_142=_arg_method_schema.method_return.clone();let _object_16=_arg_fs.line_number.clone();let _object_17=_arg_fs.column_number.clone();let _object_21=_arg_fs.file.clone();let _object_22=_arg_fs.offset.clone();let _object_23=_arg_fs.offset_end.clone();let _object_24=_arg_namespaces.namespace_block.clone();let _object_25=_arg_namespaces.namespace.clone();let _object_3=_arg_base.code.clone();let _object_6=_arg_base.name.clone();let _object_8=_arg_base.parser_type_name.clone();let info_8=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Ast$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Ast".to_owned(),schema_name:"Ast".to_owned(), doc_index:7,description:"\nThe Abstract Syntax Tree (AST) Layer provides syntax trees for all compilation units.\nAll nodes of the tree inherit from the same base class (`AST_NODE`) and are connected\nto their child nodes via outgoing `AST` edges.\n\nSyntax trees are typed, that is, when possible, types for all expressions are stored\nin the tree. Moreover, common control structure types are defined in the specification,\nmaking it possible to translate trees into corresponding control flow graphs if only\nthese common control structure types are used, possibly by desugaring on the side of\nthe language frontend. For cases where this is not an option,\nthe AST specification provides means of storing language-dependent information in the\nAST that can be interpreted by language-dependent control flow construction passes.\n\nThis layer MUST be created by the frontend.\n\n".to_owned(),provided_by_frontend:true};// CPG_CONSTRUCTION_OP:298
let _object_143=builder.add_property("ORDER".to_owned(),ValueType::Int,"This integer indicates the position of the node among\nits siblings in the AST. The left-most child has an\norder of 0.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:299
_object_143.mandatory(Scalar::Int { decimal:"-1".to_owned() })?;
// CPG_CONSTRUCTION_OP:300
_object_143.proto_id(4);
// CPG_CONSTRUCTION_OP:301
let _object_144=builder.add_node_base_type("AST_NODE".to_owned(),"This is the base type for all nodes of the abstract syntax tree (AST). An AST\nnode has a `CODE` and an `ORDER` field. The `CODE` field contains the\ncode (verbatim) represented by the AST node. The `ORDER` field contains the\nnodes position among its siblings, encoded as an integer where the left most\nsibling has the position `0`.\n\nAST nodes contain optional `LINE_NUMBER` and `COLUMN_NUMBER` fields. For\nsource-based frontends, these fields contain the start line number and\nstart column number of the code represented by the node.\nFor machine-code-based and bytecode-based frontends, `LINE_NUMBER` contains\nthe address at which the code starts while `COLUMN_NUMBER` is undefined.\n\nThe optional `OFFSET` and `OFFSET_END` specify the start\nand exclusive end position of the code belonging to a node within the corresponding\n`FILE` nodes `CONTENT` property.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:302
_object_144.add_property(&_object_143);
// CPG_CONSTRUCTION_OP:303
_object_144.add_property(&_object_3);
// CPG_CONSTRUCTION_OP:304
_object_144.add_property(&_object_16);
// CPG_CONSTRUCTION_OP:305
_object_144.add_property(&_object_17);
// CPG_CONSTRUCTION_OP:306
_object_144.add_property(&_object_22);
// CPG_CONSTRUCTION_OP:307
_object_144.add_property(&_object_23);
// CPG_CONSTRUCTION_OP:308
_object_138.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:309
_object_140.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:310
_object_141.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:311
_object_129.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:312
_object_132.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:313
_object_130.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:314
_object_131.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:315
_object_21.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:316
_object_24.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:317
let _object_145=builder.add_node_type("BLOCK".to_owned(),"This node represents a compound statement. Compound statements are used in many languages to allow\ngrouping a sequence of statements. For example, in C and Java, compound statements\nare statements enclosed by curly braces. Function/Method bodies are compound\nstatements. We do not use the term \"compound statement\" because \"statement\" would\nimply that the block does not yield a value upon evaluation, that is, that it is\nnot an expression. This is true in languages such as C and Java, but not for languages\nsuch as Scala where the value of the block is given by that of the last expression it\ncontains. In fact, the Scala grammar uses the term \"BlockExpr\" (short for\n\"block expression\") to describe what in the CPG we call \"Block\".",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:318
_object_145.proto_id(31);
// CPG_CONSTRUCTION_OP:319
_object_145.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:320
let _object_146=builder.add_node_type("LITERAL".to_owned(),"This node represents a literal such as an integer or string constant. Literals\nare symbols included in the code in verbatim form and which are immutable.\nThe `TYPE_FULL_NAME` field stores the literal's fully-qualified type name,\ne.g., `java.lang.Integer`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:321
_object_146.proto_id(8);
// CPG_CONSTRUCTION_OP:322
_object_146.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:323
_object_146.primary_key(Some(&_object_3));
// CPG_CONSTRUCTION_OP:324
let _object_147=builder.add_node_type("LOCAL".to_owned(),"This node represents a local variable. Its fully qualified type name is stored\nin the `TYPE_FULL_NAME` field and its name in the `NAME` field. The `CODE` field\ncontains the entire local variable declaration without initialization, e.g., for\n`int x = 10;`, it contains `int x`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:325
_object_147.proto_id(23);
// CPG_CONSTRUCTION_OP:326
_object_147.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:327
_object_147.extend(&_object_13)?;_object_147.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:328
_object_147.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:329
let _object_148=builder.add_node_type("IDENTIFIER".to_owned(),"This node represents an identifier as used when referring to a variable by name.\nIt holds the identifier's name in the `NAME` field and its fully-qualified type\nname in `TYPE_FULL_NAME`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:330
_object_148.proto_id(27);
// CPG_CONSTRUCTION_OP:331
_object_148.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:332
_object_148.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:333
_object_148.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:334
let _object_149=builder.add_property("CANONICAL_NAME".to_owned(),ValueType::String,"This field holds the canonical name of a `FIELD_IDENTIFIER`. It is typically\nidentical to the CODE field, but canonicalized according to source language\nsemantics. Human readable names are preferable. `FIELD_IDENTIFIER` nodes must\nshare identical `CANONICAL_NAME` if and\nonly if they alias, e.g., in C-style unions (if the aliasing relationship is\nunknown or there are partial overlaps, then one must make a reasonable guess,\nand trade off between false negatives and false positives).",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:335
_object_149.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:336
_object_149.proto_id(2001092);
// CPG_CONSTRUCTION_OP:337
let _object_150=builder.add_node_type("FIELD_IDENTIFIER".to_owned(),"This node represents the field accessed in a field access, e.g., in\n`a.b`, it represents `b`. The field name as it occurs in the code is\nstored in the `CODE` field. This may mean that the `CODE` field holds\nan expression. The `CANONICAL_NAME` field MAY contain the same value is\nthe `CODE` field but SHOULD contain the normalized name that results\nfrom evaluating `CODE` as an expression if such an evaluation is\npossible for the language frontend. The objective is to store an identifier\nin `CANONICAL_NAME` that is the same for two nodes iff they refer to the\nsame field, regardless of whether they use the same expression to reference\nit.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:338
_object_150.proto_id(2001081);
// CPG_CONSTRUCTION_OP:339
_object_150.add_property(&_object_149);
// CPG_CONSTRUCTION_OP:340
let _object_151=builder.add_property("MODIFIER_TYPE".to_owned(),ValueType::String,"The modifier type is a free-form string. The following are known modifier types:\n`STATIC`, `PUBLIC`, `PROTECTED`, `PRIVATE`, `ABSTRACT`, `NATIVE`, `CONSTRUCTOR`, `VIRTUAL`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:341
_object_151.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:342
_object_151.proto_id(26);
// CPG_CONSTRUCTION_OP:343
let constant_152=builder.constant(Constant{name:"STATIC".to_owned(),value:"STATIC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The static modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:344
constant_152.proto_id(1);
// CPG_CONSTRUCTION_OP:345
let constant_153=builder.constant(Constant{name:"PUBLIC".to_owned(),value:"PUBLIC".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The public modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:346
constant_153.proto_id(2);
// CPG_CONSTRUCTION_OP:347
let constant_154=builder.constant(Constant{name:"PROTECTED".to_owned(),value:"PROTECTED".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The protected modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:348
constant_154.proto_id(3);
// CPG_CONSTRUCTION_OP:349
let constant_155=builder.constant(Constant{name:"PRIVATE".to_owned(),value:"PRIVATE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The private modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:350
constant_155.proto_id(4);
// CPG_CONSTRUCTION_OP:351
let constant_156=builder.constant(Constant{name:"ABSTRACT".to_owned(),value:"ABSTRACT".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The abstract modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:352
constant_156.proto_id(5);
// CPG_CONSTRUCTION_OP:353
let constant_157=builder.constant(Constant{name:"NATIVE".to_owned(),value:"NATIVE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The native modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:354
constant_157.proto_id(6);
// CPG_CONSTRUCTION_OP:355
let constant_158=builder.constant(Constant{name:"CONSTRUCTOR".to_owned(),value:"CONSTRUCTOR".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The constructor modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:356
constant_158.proto_id(7);
// CPG_CONSTRUCTION_OP:357
let constant_159=builder.constant(Constant{name:"VIRTUAL".to_owned(),value:"VIRTUAL".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The virtual modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:358
constant_159.proto_id(8);
// CPG_CONSTRUCTION_OP:359
let constant_160=builder.constant(Constant{name:"INTERNAL".to_owned(),value:"INTERNAL".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The internal modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:360
constant_160.proto_id(9);
// CPG_CONSTRUCTION_OP:361
let constant_161=builder.constant(Constant{name:"FINAL".to_owned(),value:"FINAL".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The final modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:362
constant_161.proto_id(10);
// CPG_CONSTRUCTION_OP:363
let constant_162=builder.constant(Constant{name:"READONLY".to_owned(),value:"READONLY".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("The readonly modifier".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:364
constant_162.proto_id(11);
// CPG_CONSTRUCTION_OP:365
let constant_163=builder.constant(Constant{name:"MODULE".to_owned(),value:"MODULE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Indicate that a method defines a module in the sense e.g. a python module does with the creation of a module object".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:366
constant_163.proto_id(12);
// CPG_CONSTRUCTION_OP:367
let constant_164=builder.constant(Constant{name:"LAMBDA".to_owned(),value:"LAMBDA".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Indicate that a method is an anonymous function, lambda, or closure".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:368
constant_164.proto_id(13);
// CPG_CONSTRUCTION_OP:369
builder.add_constants("ModifierTypes".to_owned(),[constant_152.clone(),constant_153.clone(),constant_154.clone(),constant_155.clone(),constant_156.clone(),constant_157.clone(),constant_158.clone(),constant_159.clone(),constant_160.clone(),constant_161.clone(),constant_162.clone(),constant_163.clone(),constant_164.clone(),]);
// CPG_CONSTRUCTION_OP:370
let _object_165=builder.add_node_type("MODIFIER".to_owned(),"This field represents a (language-dependent) modifier such as `static`, `private`\nor `public`. Unlike most other AST nodes, it is NOT an expression, that is, it\ncannot be evaluated and cannot be passed as an argument in function calls.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:371
_object_165.proto_id(300);
// CPG_CONSTRUCTION_OP:372
_object_165.add_property(&_object_151);
// CPG_CONSTRUCTION_OP:373
_object_165.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:374
let _object_166=builder.add_node_type("JUMP_TARGET".to_owned(),"A jump target is any location in the code that has been specifically marked\nas the target of a jump, e.g., via a label. The `NAME` field holds the name of\nthe label while the `PARSER_TYPE_NAME` field holds the name of language construct\nthat this jump target is created from, e.g., \"Label\".",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:375
_object_166.proto_id(340);
// CPG_CONSTRUCTION_OP:376
_object_166.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:377
_object_166.add_property(&_object_8);
// CPG_CONSTRUCTION_OP:378
_object_166.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:379
let _object_167=builder.add_node_type("JUMP_LABEL".to_owned(),"A jump label specifies the label and thus the JUMP_TARGET of control structures\nBREAK and CONTINUE. The `NAME` field holds the name of the label while the\n`PARSER_TYPE_NAME` field holds the name of language construct that this jump\nlabel is created from, e.g., \"Label\".",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:380
_object_167.proto_id(341);
// CPG_CONSTRUCTION_OP:381
_object_167.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:382
_object_167.add_property(&_object_8);
// CPG_CONSTRUCTION_OP:383
_object_167.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:384
let _object_168=builder.add_node_type("METHOD_REF".to_owned(),"This node represents a reference to a method/function/procedure as it\nappears when a method is passed as an argument in a call. The `METHOD_FULL_NAME`\nfield holds the fully-qualified name of the referenced method and the\n`TYPE_FULL_NAME` holds its fully-qualified type name.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:385
_object_168.proto_id(333);
// CPG_CONSTRUCTION_OP:386
_object_168.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:387
let _object_169=builder.add_node_type("TYPE_REF".to_owned(),"Reference to a type/class",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:388
_object_169.proto_id(335);
// CPG_CONSTRUCTION_OP:389
_object_169.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:390
let _object_170=builder.add_node_type("RETURN".to_owned(),"This node represents a return instruction, e.g., `return x`. Note that it does\nNOT represent a formal return parameter as formal return parameters are\nrepresented via `METHOD_RETURN` nodes.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:391
_object_170.proto_id(30);
// CPG_CONSTRUCTION_OP:392
_object_170.starter_name(Some("ret".to_owned()));
// CPG_CONSTRUCTION_OP:393
_object_170.primary_key(Some(&_object_3));
// CPG_CONSTRUCTION_OP:394
let _object_171=builder.add_property("CONTROL_STRUCTURE_TYPE".to_owned(),ValueType::String,"The `CONTROL_STRUCTURE_TYPE` field indicates which kind of control structure\na `CONTROL_STRUCTURE` node represents. The available types are the following:\n BREAK, CONTINUE, DO, WHILE, FOR, GOTO, IF, ELSE, TRY, THROW and SWITCH.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:395
_object_171.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:396
_object_171.proto_id(27);
// CPG_CONSTRUCTION_OP:397
let constant_172=builder.constant(Constant{name:"BREAK".to_owned(),value:"BREAK".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a break statement. Labeled breaks are expected to have a JUMP_LABEL\nnode AST child with ORDER 1".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:398
constant_172.proto_id(1);
// CPG_CONSTRUCTION_OP:399
let constant_173=builder.constant(Constant{name:"CONTINUE".to_owned(),value:"CONTINUE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a continue statement. Labeled continues are expected to have a JUMP_LABEL\nnode AST child with ORDER 1".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:400
constant_173.proto_id(2);
// CPG_CONSTRUCTION_OP:401
let constant_174=builder.constant(Constant{name:"WHILE".to_owned(),value:"WHILE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a while statement".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:402
constant_174.proto_id(3);
// CPG_CONSTRUCTION_OP:403
let constant_175=builder.constant(Constant{name:"DO".to_owned(),value:"DO".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a do statement".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:404
constant_175.proto_id(4);
// CPG_CONSTRUCTION_OP:405
let constant_176=builder.constant(Constant{name:"FOR".to_owned(),value:"FOR".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a for statement".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:406
constant_176.proto_id(5);
// CPG_CONSTRUCTION_OP:407
let constant_177=builder.constant(Constant{name:"GOTO".to_owned(),value:"GOTO".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a goto statement".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:408
constant_177.proto_id(6);
// CPG_CONSTRUCTION_OP:409
let constant_178=builder.constant(Constant{name:"IF".to_owned(),value:"IF".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents an if statement".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:410
constant_178.proto_id(7);
// CPG_CONSTRUCTION_OP:411
let constant_179=builder.constant(Constant{name:"ELSE".to_owned(),value:"ELSE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents an else statement".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:412
constant_179.proto_id(8);
// CPG_CONSTRUCTION_OP:413
let constant_180=builder.constant(Constant{name:"SWITCH".to_owned(),value:"SWITCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a switch statement".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:414
constant_180.proto_id(9);
// CPG_CONSTRUCTION_OP:415
let constant_181=builder.constant(Constant{name:"TRY".to_owned(),value:"TRY".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a try statement".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:416
constant_181.proto_id(10);
// CPG_CONSTRUCTION_OP:417
let constant_182=builder.constant(Constant{name:"THROW".to_owned(),value:"THROW".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a throw statement".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:418
constant_182.proto_id(11);
// CPG_CONSTRUCTION_OP:419
let constant_183=builder.constant(Constant{name:"MATCH".to_owned(),value:"MATCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a match expression".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:420
constant_183.proto_id(12);
// CPG_CONSTRUCTION_OP:421
let constant_184=builder.constant(Constant{name:"YIELD".to_owned(),value:"YIELD".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a yield expression".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:422
constant_184.proto_id(13);
// CPG_CONSTRUCTION_OP:423
let constant_185=builder.constant(Constant{name:"CATCH".to_owned(),value:"CATCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a catch clause".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:424
constant_185.proto_id(14);
// CPG_CONSTRUCTION_OP:425
let constant_186=builder.constant(Constant{name:"FINALLY".to_owned(),value:"FINALLY".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Represents a finally clause".to_owned()),schema_info:Some(info_8.clone())});
// CPG_CONSTRUCTION_OP:426
constant_186.proto_id(15);
// CPG_CONSTRUCTION_OP:427
builder.add_constants("ControlStructureTypes".to_owned(),[constant_172.clone(),constant_173.clone(),constant_174.clone(),constant_175.clone(),constant_176.clone(),constant_177.clone(),constant_178.clone(),constant_179.clone(),constant_180.clone(),constant_181.clone(),constant_182.clone(),constant_183.clone(),constant_184.clone(),constant_185.clone(),constant_186.clone(),]);
// CPG_CONSTRUCTION_OP:428
let _object_187=builder.add_node_type("CONTROL_STRUCTURE".to_owned(),"This node represents a control structure as introduced by control structure\nstatements as well as conditional and unconditional jumps. Its type is stored in the\n`CONTROL_STRUCTURE_TYPE` field to be one of several pre-defined types. These types\n are used in the construction of the control flow layer, making it possible to\n generate the control flow layer from the abstract syntax tree layer automatically.\n\nIn addition to the `CONTROL_STRUCTURE_TYPE` field, the `PARSER_TYPE_NAME` field\nMAY be used by frontends to store the name of the control structure as emitted by\nthe parser or disassembler, however, the value of this field is not relevant\nfor construction of the control flow layer.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:429
_object_187.proto_id(339);
// CPG_CONSTRUCTION_OP:430
_object_187.add_property(&_object_8);
// CPG_CONSTRUCTION_OP:431
_object_187.add_property(&_object_171);
// CPG_CONSTRUCTION_OP:432
let _object_188=builder.add_node_type("UNKNOWN".to_owned(),"Any AST node that the frontend would like to include in the AST but for\nwhich no suitable AST node is specified in the CPG specification may be\nincluded using a node of type `UNKNOWN`.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:433
_object_188.proto_id(44);
// CPG_CONSTRUCTION_OP:434
_object_188.add_property(&_object_8);
// CPG_CONSTRUCTION_OP:435
_object_188.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:436
let _object_189=builder.add_edge_type("AST".to_owned(),"This edge connects a parent node to its child in the syntax tree.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:437
_object_189.proto_id(3);
// CPG_CONSTRUCTION_OP:438
let _object_190=builder.add_edge_type("CONDITION".to_owned(),"The edge connects control structure nodes to the expressions that holds their conditions.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:439
_object_190.proto_id(56);
// CPG_CONSTRUCTION_OP:440
let _object_191=builder.add_edge_type("TRUE_BODY".to_owned(),"The edge connects control structure nodes to their true branch/body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:441
_object_191.proto_id(60);
// CPG_CONSTRUCTION_OP:442
let _object_192=builder.add_edge_type("FALSE_BODY".to_owned(),"The edge connects control structure nodes to their false branch/body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:443
_object_192.proto_id(61);
// CPG_CONSTRUCTION_OP:444
let _object_193=builder.add_edge_type("DO_BODY".to_owned(),"The edge connects do-while control structure nodes to their body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:445
_object_193.proto_id(62);
// CPG_CONSTRUCTION_OP:446
let _object_194=builder.add_edge_type("FOR_INIT".to_owned(),"The edge connects for-loop control structure nodes to their initialization expression(s).",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:447
_object_194.proto_id(63);
// CPG_CONSTRUCTION_OP:448
let _object_195=builder.add_edge_type("FOR_UPDATE".to_owned(),"The edge connects for-loop control structure nodes to their update/step expression(s).",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:449
_object_195.proto_id(64);
// CPG_CONSTRUCTION_OP:450
let _object_196=builder.add_edge_type("FOR_BODY".to_owned(),"The edge connects for-loop control structure nodes to their body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:451
_object_196.proto_id(65);
// CPG_CONSTRUCTION_OP:452
let _object_197=builder.add_edge_type("TRY_BODY".to_owned(),"The edge connects try control structure nodes to their try body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:453
_object_197.proto_id(66);
// CPG_CONSTRUCTION_OP:454
let _object_198=builder.add_edge_type("CATCH_BODY".to_owned(),"The edge connects try control structure nodes to catch/handler bodies.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:455
_object_198.proto_id(67);
// CPG_CONSTRUCTION_OP:456
let _object_199=builder.add_edge_type("FINALLY_BODY".to_owned(),"The edge connects try control structure nodes to their finally body.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:457
_object_199.proto_id(68);
// CPG_CONSTRUCTION_OP:458
let _object_200=builder.add_edge_type("JUMP_ARGUMENT".to_owned(),"The edge connects jump-like control structures to the node encoding their jump target.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:459
_object_200.proto_id(69);
// CPG_CONSTRUCTION_OP:460
_object_21.add_out_edge(&_object_189,&_object_24,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:461
_object_132.add_out_edge(&_object_189,&_object_165,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:462
_object_133.add_out_edge(&_object_189,&_object_131,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:463
_object_129.add_out_edge(&_object_189,&_object_130,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:464
_object_129.add_out_edge(&_object_189,&_object_132,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"typeDecl".to_owned(),in_doc:"The type declaration this member is defined in".to_owned()});
// CPG_CONSTRUCTION_OP:465
_object_129.add_out_edge(&_object_189,&_object_165,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:466
_object_138.add_out_edge(&_object_189,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::One, out_step:"methodReturn".to_owned(),out_doc:"Formal return parameters".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:467
_object_138.add_out_edge(&_object_189,&_object_140,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"parameter".to_owned(),out_doc:"Parameters of the method".to_owned(),in_step:"method".to_owned(),in_doc:"Traverse to method associated with this formal parameter".to_owned()});
// CPG_CONSTRUCTION_OP:468
_object_138.add_out_edge(&_object_189,&_object_165,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:469
_object_138.add_out_edge(&_object_189,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::One, out_step:"block".to_owned(),out_doc:"Root of the abstract syntax tree".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:470
_object_138.add_out_edge(&_object_189,&_object_130,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:471
_object_170.add_out_edge(&_object_189,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:472
_object_170.add_out_edge(&_object_189,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:473
_object_170.add_out_edge(&_object_189,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:474
_object_170.add_out_edge(&_object_189,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:475
_object_170.add_out_edge(&_object_189,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:476
_object_170.add_out_edge(&_object_189,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:477
_object_170.add_out_edge(&_object_189,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:478
_object_170.add_out_edge(&_object_189,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:479
_object_170.add_out_edge(&_object_189,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:480
_object_145.add_out_edge(&_object_189,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:481
_object_145.add_out_edge(&_object_189,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:482
_object_145.add_out_edge(&_object_189,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:483
_object_145.add_out_edge(&_object_189,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:484
_object_145.add_out_edge(&_object_189,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:485
_object_145.add_out_edge(&_object_189,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:486
_object_145.add_out_edge(&_object_189,&_object_147,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"local".to_owned(),out_doc:"Traverse to locals of this block.".to_owned(),in_step:"definingBlock".to_owned(),in_doc:"The block in which local is declared.".to_owned()});
// CPG_CONSTRUCTION_OP:487
_object_145.add_out_edge(&_object_189,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:488
_object_145.add_out_edge(&_object_189,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:489
_object_145.add_out_edge(&_object_189,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:490
_object_187.add_out_edge(&_object_189,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:491
_object_187.add_out_edge(&_object_189,&_object_165,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:492
_object_187.add_out_edge(&_object_189,&_object_147,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:493
_object_187.add_out_edge(&_object_189,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:494
_object_187.add_out_edge(&_object_189,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:495
_object_187.add_out_edge(&_object_189,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:496
_object_187.add_out_edge(&_object_189,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:497
_object_187.add_out_edge(&_object_189,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:498
_object_187.add_out_edge(&_object_189,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:499
_object_187.add_out_edge(&_object_189,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:500
_object_187.add_out_edge(&_object_189,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:501
_object_187.add_out_edge(&_object_189,&_object_167,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:502
_object_188.add_out_edge(&_object_189,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:503
_object_188.add_out_edge(&_object_189,&_object_165,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:504
_object_188.add_out_edge(&_object_189,&_object_147,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:505
_object_188.add_out_edge(&_object_189,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:506
_object_188.add_out_edge(&_object_189,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:507
_object_188.add_out_edge(&_object_189,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:508
_object_188.add_out_edge(&_object_189,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:509
_object_188.add_out_edge(&_object_189,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:510
_object_188.add_out_edge(&_object_189,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:511
_object_188.add_out_edge(&_object_189,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:512
_object_188.add_out_edge(&_object_189,&_object_132,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:513
_object_140.add_out_edge(&_object_189,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:514
_object_24.add_out_edge(&_object_189,&_object_129,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"namespaceBlock".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:515
_object_24.add_out_edge(&_object_189,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:516
_object_25.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:517
_object_187.add_out_edge(&_object_190,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:518
_object_187.add_out_edge(&_object_190,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:519
_object_187.add_out_edge(&_object_190,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:520
_object_187.add_out_edge(&_object_190,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:521
_object_187.add_out_edge(&_object_190,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:522
_object_187.add_out_edge(&_object_190,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:523
_object_187.add_out_edge(&_object_190,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:524
_object_187.add_out_edge(&_object_190,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:525
_object_187.add_out_edge(&_object_190,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:526
_object_187.add_out_edge(&_object_190,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:527
_object_187.add_out_edge(&_object_191,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:528
_object_187.add_out_edge(&_object_191,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:529
_object_187.add_out_edge(&_object_191,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:530
_object_187.add_out_edge(&_object_191,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:531
_object_187.add_out_edge(&_object_191,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:532
_object_187.add_out_edge(&_object_191,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:533
_object_187.add_out_edge(&_object_191,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:534
_object_187.add_out_edge(&_object_191,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:535
_object_187.add_out_edge(&_object_191,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:536
_object_187.add_out_edge(&_object_192,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:537
_object_187.add_out_edge(&_object_192,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:538
_object_187.add_out_edge(&_object_192,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:539
_object_187.add_out_edge(&_object_192,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:540
_object_187.add_out_edge(&_object_192,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:541
_object_187.add_out_edge(&_object_192,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:542
_object_187.add_out_edge(&_object_192,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:543
_object_187.add_out_edge(&_object_192,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:544
_object_187.add_out_edge(&_object_192,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:545
_object_187.add_out_edge(&_object_193,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:546
_object_187.add_out_edge(&_object_193,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:547
_object_187.add_out_edge(&_object_193,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:548
_object_187.add_out_edge(&_object_193,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:549
_object_187.add_out_edge(&_object_193,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:550
_object_187.add_out_edge(&_object_193,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:551
_object_187.add_out_edge(&_object_193,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:552
_object_187.add_out_edge(&_object_193,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:553
_object_187.add_out_edge(&_object_193,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:554
_object_187.add_out_edge(&_object_194,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:555
_object_187.add_out_edge(&_object_194,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:556
_object_187.add_out_edge(&_object_194,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:557
_object_187.add_out_edge(&_object_194,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:558
_object_187.add_out_edge(&_object_194,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:559
_object_187.add_out_edge(&_object_194,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:560
_object_187.add_out_edge(&_object_194,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:561
_object_187.add_out_edge(&_object_194,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:562
_object_187.add_out_edge(&_object_194,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:563
_object_187.add_out_edge(&_object_195,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:564
_object_187.add_out_edge(&_object_195,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:565
_object_187.add_out_edge(&_object_195,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:566
_object_187.add_out_edge(&_object_195,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:567
_object_187.add_out_edge(&_object_195,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:568
_object_187.add_out_edge(&_object_195,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:569
_object_187.add_out_edge(&_object_195,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:570
_object_187.add_out_edge(&_object_195,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:571
_object_187.add_out_edge(&_object_195,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:572
_object_187.add_out_edge(&_object_196,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:573
_object_187.add_out_edge(&_object_196,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:574
_object_187.add_out_edge(&_object_196,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:575
_object_187.add_out_edge(&_object_196,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:576
_object_187.add_out_edge(&_object_196,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:577
_object_187.add_out_edge(&_object_196,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:578
_object_187.add_out_edge(&_object_196,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:579
_object_187.add_out_edge(&_object_196,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:580
_object_187.add_out_edge(&_object_196,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:581
_object_187.add_out_edge(&_object_197,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:582
_object_187.add_out_edge(&_object_197,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:583
_object_187.add_out_edge(&_object_197,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:584
_object_187.add_out_edge(&_object_197,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:585
_object_187.add_out_edge(&_object_197,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:586
_object_187.add_out_edge(&_object_197,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:587
_object_187.add_out_edge(&_object_197,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:588
_object_187.add_out_edge(&_object_197,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:589
_object_187.add_out_edge(&_object_197,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:590
_object_187.add_out_edge(&_object_198,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:591
_object_187.add_out_edge(&_object_198,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:592
_object_187.add_out_edge(&_object_198,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:593
_object_187.add_out_edge(&_object_198,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:594
_object_187.add_out_edge(&_object_198,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:595
_object_187.add_out_edge(&_object_198,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:596
_object_187.add_out_edge(&_object_198,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:597
_object_187.add_out_edge(&_object_198,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:598
_object_187.add_out_edge(&_object_198,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:599
_object_187.add_out_edge(&_object_199,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:600
_object_187.add_out_edge(&_object_199,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:601
_object_187.add_out_edge(&_object_199,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:602
_object_187.add_out_edge(&_object_199,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:603
_object_187.add_out_edge(&_object_199,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:604
_object_187.add_out_edge(&_object_199,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:605
_object_187.add_out_edge(&_object_199,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:606
_object_187.add_out_edge(&_object_199,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:607
_object_187.add_out_edge(&_object_199,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:608
_object_187.add_out_edge(&_object_200,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:609
_object_187.add_out_edge(&_object_200,&_object_167,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:610
_object_129.add_out_edge(&_object_189,&_object_129,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:611
_object_129.add_out_edge(&_object_189,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:612
_object_138.add_out_edge(&_object_189,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"method".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:613
_object_138.add_out_edge(&_object_189,&_object_129,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:614
_object_138.add_out_edge(&_object_189,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:615
let _object_201=builder.add_node_base_type("CALL_REPR".to_owned(),"This is the base class of `CALL` that language implementers may safely ignore.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:616
_object_201.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:617
_object_201.add_property(&_object_137);
// CPG_CONSTRUCTION_OP:618
let _object_202=builder.add_node_type("CALL".to_owned(),"A (function/method/procedure) call. The `METHOD_FULL_NAME` property is the name of the\ninvoked method (the callee) while the `TYPE_FULL_NAME` is its return type, and\ntherefore, the return type of the call when viewing it as an expression. For\nlanguages like Javascript, it is common that we may know the (short-) name\nof the invoked method, but we do not know at compile time which method\nwill actually be invoked, e.g., because it depends on a dynamic import.\nIn this case, we leave `METHOD_FULL_NAME` blank but at least fill out `NAME`,\nwhich contains the method's (short-) name and `SIGNATURE`, which contains\nany information we may have about the types of arguments and return value.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:619
_object_202.proto_id(15);
// CPG_CONSTRUCTION_OP:620
_object_202.extend(&_object_201)?;
// CPG_CONSTRUCTION_OP:621
_object_202.add_property(&_object_125);
// CPG_CONSTRUCTION_OP:622
_object_202.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:623
let _object_203=builder.add_node_base_type("EXPRESSION".to_owned(),"`EXPRESSION` is the base class for all nodes that represent code pieces\nthat can be evaluated.\n\n Expression may be arguments in method calls. For method calls that do\n not involved named parameters, the `ARGUMENT_INDEX` field indicates at\n which position in the argument list the expression occurs, e.g., an\n `ARGUMENT_INDEX` of 1 indicates that the expression is the first argument\n in a method call. For calls that employ named parameters, `ARGUMENT_INDEX`\n is set to -1 and the `ARGUMENT_NAME` fields holds the name of the parameter.",Some(info_8.clone()));
// CPG_CONSTRUCTION_OP:624
_object_203.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:625
_object_150.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:626
_object_148.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:627
_object_146.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:628
_object_145.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:629
_object_187.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:630
_object_168.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:631
_object_169.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:632
_object_170.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:633
_object_188.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:634
_object_202.add_out_edge(&_object_189,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:635
_object_202.add_out_edge(&_object_189,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:636
_object_202.add_out_edge(&_object_189,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:637
_object_202.add_out_edge(&_object_189,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:638
_object_202.add_out_edge(&_object_189,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:639
_object_202.add_out_edge(&_object_189,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:640
_object_202.add_out_edge(&_object_189,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:641
_object_202.add_out_edge(&_object_189,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:642
_object_202.add_out_edge(&_object_189,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:643
_object_170.add_out_edge(&_object_189,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:644
_object_187.add_out_edge(&_object_189,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:645
_object_188.add_out_edge(&_object_189,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:646
_object_187.add_out_edge(&_object_190,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:647
_object_187.add_out_edge(&_object_191,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:648
_object_187.add_out_edge(&_object_192,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:649
_object_187.add_out_edge(&_object_193,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:650
_object_187.add_out_edge(&_object_194,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:651
_object_187.add_out_edge(&_object_195,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:652
_object_187.add_out_edge(&_object_196,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:653
_object_187.add_out_edge(&_object_197,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:654
_object_187.add_out_edge(&_object_198,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:655
_object_187.add_out_edge(&_object_199,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:656
_object_145.add_out_edge(&_object_189,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:657
_object_148.add_out_edge(&_object_14,&_object_147,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"referencingIdentifiers".to_owned(),in_doc:"Places (identifier) where this local is being referenced".to_owned()});
// CPG_CONSTRUCTION_OP:658
_object_148.add_out_edge(&_object_14,&_object_140,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"referencingIdentifiers".to_owned(),in_doc:"Places (identifier) where this parameter is being referenced".to_owned()});
// CPG_CONSTRUCTION_OP:659
_object_24.add_out_edge(&_object_14,&_object_25,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{order:_object_143,ast_node:_object_144,block:_object_145,literal:_object_146,local:_object_147,identifier:_object_148,canonical_name:_object_149,field_identifier:_object_150,modifier_type:_object_151,modifier_types:vec![constant_152.clone(),constant_153.clone(),constant_154.clone(),constant_155.clone(),constant_156.clone(),constant_157.clone(),constant_158.clone(),constant_159.clone(),constant_160.clone(),constant_161.clone(),constant_162.clone(),constant_163.clone(),constant_164.clone(),],modifier:_object_165,jump_target:_object_166,jump_label:_object_167,method_ref:_object_168,type_ref:_object_169,ret:_object_170,control_structure_type:_object_171,control_structure_types:vec![constant_172.clone(),constant_173.clone(),constant_174.clone(),constant_175.clone(),constant_176.clone(),constant_177.clone(),constant_178.clone(),constant_179.clone(),constant_180.clone(),constant_181.clone(),constant_182.clone(),constant_183.clone(),constant_184.clone(),constant_185.clone(),constant_186.clone(),],control_structure:_object_187,unknown:_object_188,ast:_object_189,condition:_object_190,true_body:_object_191,false_body:_object_192,do_body:_object_193,for_init:_object_194,for_update:_object_195,for_body:_object_196,try_body:_object_197,catch_body:_object_198,finally_body:_object_199,jump_argument:_object_200,call_repr:_object_201,call_node:_object_202,expression:_object_203,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_namespaces:&NamespaceSchema,_arg_method_schema:&MethodSchema,_arg_type_schema:&TypeSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{Self::new(builder,_arg_base,_arg_namespaces,_arg_method_schema,_arg_type_schema,_arg_fs,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Ast$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Ast".to_owned(),schema_name:"Ast".to_owned(), doc_index:7,description:"\nThe Abstract Syntax Tree (AST) Layer provides syntax trees for all compilation units.\nAll nodes of the tree inherit from the same base class (`AST_NODE`) and are connected\nto their child nodes via outgoing `AST` edges.\n\nSyntax trees are typed, that is, when possible, types for all expressions are stored\nin the tree. Moreover, common control structure types are defined in the specification,\nmaking it possible to translate trees into corresponding control flow graphs if only\nthese common control structure types are used, possibly by desugaring on the side of\nthe language frontend. For cases where this is not an option,\nthe AST specification provides means of storing language-dependent information in the\nAST that can be interpreted by language-dependent control flow construction passes.\n\nThis layer MUST be created by the frontend.\n\n".to_owned(),provided_by_frontend:true}} } impl SchemaBase for AstSchema { fn doc_index(&self)->i32{7} fn description(&self)->&str{"\nThe Abstract Syntax Tree (AST) Layer provides syntax trees for all compilation units.\nAll nodes of the tree inherit from the same base class (`AST_NODE`) and are connected\nto their child nodes via outgoing `AST` edges.\n\nSyntax trees are typed, that is, when possible, types for all expressions are stored\nin the tree. Moreover, common control structure types are defined in the specification,\nmaking it possible to translate trees into corresponding control flow graphs if only\nthese common control structure types are used, possibly by desugaring on the side of\nthe language frontend. For cases where this is not an option,\nthe AST specification provides means of storing language-dependent information in the\nAST that can be interpreted by language-dependent control flow construction passes.\n\nThis layer MUST be created by the frontend.\n\n"} fn provided_by_frontend(&self)->bool{true} }// CPG_SCHEMA_MODULE_END:Ast
// CPG_SCHEMA_MODULE:CallGraph
#[derive(Clone,Debug)] pub struct CallGraphSchema {pub argument_index:PropertyRef,pub argument_name:PropertyRef,pub argument_label:PropertyRef,pub method_full_name:PropertyRef,pub evaluation_strategy:PropertyRef,pub evaluation_strategies:Vec<ConstantRef>,pub dispatch_type:PropertyRef,pub call:EdgeRef,pub argument:EdgeRef,pub receiver:EdgeRef,pub dispatch_types:Vec<ConstantRef>,pub static_receiver:PropertyRef,}impl CallGraphSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_method_schema:&MethodSchema,_arg_ast_schema:&AstSchema,)->Result<Self>{let _object_138=_arg_method_schema.method.clone();let _object_140=_arg_method_schema.method_parameter_in.clone();let _object_141=_arg_method_schema.method_parameter_out.clone();let _object_142=_arg_method_schema.method_return.clone();let _object_145=_arg_ast_schema.block.clone();let _object_146=_arg_ast_schema.literal.clone();let _object_148=_arg_ast_schema.identifier.clone();let _object_150=_arg_ast_schema.field_identifier.clone();let _object_166=_arg_ast_schema.jump_target.clone();let _object_168=_arg_ast_schema.method_ref.clone();let _object_169=_arg_ast_schema.type_ref.clone();let _object_170=_arg_ast_schema.ret.clone();let _object_187=_arg_ast_schema.control_structure.clone();let _object_188=_arg_ast_schema.unknown.clone();let _object_202=_arg_ast_schema.call_node.clone();let _object_203=_arg_ast_schema.expression.clone();let info_9=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.CallGraph$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.CallGraph".to_owned(),schema_name:"CallGraph".to_owned(), doc_index:8,description:"\nThe Call Graph Layer represents call relations between methods.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:660
let _object_204=builder.add_property("ARGUMENT_INDEX".to_owned(),ValueType::Int,"AST-children of CALL nodes have an argument index, that is used to match\ncall-site arguments with callee parameters. Explicit parameters are numbered\nfrom 1 to N, while index 0 is reserved for implicit self / this parameter.\nCALLs without implicit parameter therefore have arguments starting with index 1.\nAST-children of BLOCK nodes may have an argument index as well; in this case,\nthe last argument index determines the return expression of a BLOCK expression.\nIf the `PARAMETER_NAME` field is set, then the `ARGUMENT_INDEX` field is\nignored. It is suggested to set it to -1.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:661
_object_204.mandatory(Scalar::Int { decimal:"-1".to_owned() })?;
// CPG_CONSTRUCTION_OP:662
_object_204.proto_id(40);
// CPG_CONSTRUCTION_OP:663
let _object_205=builder.add_property("ARGUMENT_NAME".to_owned(),ValueType::String,"For calls involving named parameters, the `ARGUMENT_NAME` field holds the\nname of the parameter initialized by the expression. For all other calls,\nthis field is unset.\nNote that the `ARGUMENT_NAME` should be an exact match of the NAME of a\nMETHOD_PARAMETER_{IN,OUT}. It overrides ARGUMENT_INDEX for dataflow purposes.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:664
_object_205.proto_id(130);
// CPG_CONSTRUCTION_OP:665
let _object_206=builder.add_property("ARGUMENT_LABEL".to_owned(),ValueType::String,"This field is used to keep track of the argument label for languages that support them, such as Swift.\nIt is used in addition to `ARGUMENT_INDEX` and can be used to reconstruct the original call syntax more faithfully.\nFor example, in Swift, a method call may look like `foo(arg1: 42, arg2: \"hello\")` where `arg1` and `arg2`\nare argument labels. In this case, the `ARGUMENT_LABEL` field for the first argument would be set to `arg1`\nand for the second argument it would be set to `arg2`.\nContrary to the `ARGUMENT_NAME` the label should not be expected to match the name of any parameter,\nand is not needed for dataflow purposes at all.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:666
_object_206.proto_id(129);
// CPG_CONSTRUCTION_OP:667
let _object_207=builder.add_property("METHOD_FULL_NAME".to_owned(),ValueType::String,"The FULL_NAME of a method. Used to link CALL and METHOD nodes. It is required\nto have exactly one METHOD node for each METHOD_FULL_NAME",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:668
_object_207.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:669
_object_207.proto_id(54);
// CPG_CONSTRUCTION_OP:670
_object_203.add_property(&_object_204);
// CPG_CONSTRUCTION_OP:671
_object_203.add_property(&_object_205);
// CPG_CONSTRUCTION_OP:672
_object_203.add_property(&_object_206);
// CPG_CONSTRUCTION_OP:673
let _object_208=builder.add_property("EVALUATION_STRATEGY".to_owned(),ValueType::String,"For formal method input parameters, output parameters, and return parameters,\nthis field holds the evaluation strategy, which is one of the following:\n1) `BY_REFERENCE` indicates that the parameter is passed by reference, 2)\n`BY_VALUE` indicates that it is passed by value, that is, a copy is made,\n3) `BY_SHARING` the parameter is a pointer/reference and it is shared with\nthe caller/callee. While a copy of the pointer is made, a copy of the object\nthat it points to is not made.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:674
_object_208.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:675
_object_208.proto_id(15);
// CPG_CONSTRUCTION_OP:676
let constant_209=builder.constant(Constant{name:"BY_REFERENCE".to_owned(),value:"BY_REFERENCE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("A parameter or return of a function is passed by reference which means an address is used behind the scenes".to_owned()),schema_info:Some(info_9.clone())});
// CPG_CONSTRUCTION_OP:677
constant_209.proto_id(1);
// CPG_CONSTRUCTION_OP:678
let constant_210=builder.constant(Constant{name:"BY_SHARING".to_owned(),value:"BY_SHARING".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("Only applicable to object parameter or return values. The pointer to the object is passed by value but the object itself is not copied and changes to it are thus propagated out of the method context".to_owned()),schema_info:Some(info_9.clone())});
// CPG_CONSTRUCTION_OP:679
constant_210.proto_id(2);
// CPG_CONSTRUCTION_OP:680
let constant_211=builder.constant(Constant{name:"BY_VALUE".to_owned(),value:"BY_VALUE".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("A parameter or return of a function passed by value which means a flat copy is used".to_owned()),schema_info:Some(info_9.clone())});
// CPG_CONSTRUCTION_OP:681
constant_211.proto_id(3);
// CPG_CONSTRUCTION_OP:682
builder.add_constants("EvaluationStrategies".to_owned(),[constant_209.clone(),constant_210.clone(),constant_211.clone(),]);
// CPG_CONSTRUCTION_OP:683
_object_140.add_property(&_object_208);
// CPG_CONSTRUCTION_OP:684
_object_142.add_property(&_object_208);
// CPG_CONSTRUCTION_OP:685
_object_141.add_property(&_object_208);
// CPG_CONSTRUCTION_OP:686
let _object_212=builder.add_property("DISPATCH_TYPE".to_owned(),ValueType::String,"This field holds the dispatch type of a call, which is either `STATIC_DISPATCH` or\n`DYNAMIC_DISPATCH`. For statically dispatched method calls, the call target is known\nat compile time while for dynamically dispatched calls, it can only be determined at\nruntime as it may depend on the type of an object (as is the case for virtual method\ncalls) or calculation of an offset.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:687
_object_212.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:688
_object_212.proto_id(25);
// CPG_CONSTRUCTION_OP:689
let _object_213=builder.add_edge_type("CALL".to_owned(),"This edge connects call sites, i.e., nodes with the type `CALL`, to the\nmethod node that represent the method they invoke. The frontend MAY create\n`CALL` edges but is not required to do so. Instead, of the `METHOD_FULL_NAME`\nfield of the `CALL` node is set correctly, `CALL` edges are created\nautomatically as the CPG is first loaded.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:690
_object_213.with_default_accessor_name(Some("STATIC_CALL".to_owned()));
// CPG_CONSTRUCTION_OP:691
_object_213.proto_id(6);
// CPG_CONSTRUCTION_OP:692
let _object_214=builder.add_edge_type("ARGUMENT".to_owned(),"Argument edges connect call sites (node type `CALL`) to their arguments\n(node type `EXPRESSION`) as well as `RETURN` nodes to the expressions\nthat return.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:693
_object_214.proto_id(156);
// CPG_CONSTRUCTION_OP:694
let _object_215=builder.add_edge_type("RECEIVER".to_owned(),"Similar to `ARGUMENT` edges, `RECEIVER` edges connect call sites\nto their receiver arguments. A receiver argument is the object on\nwhich a method operates, that is, it is the expression that is\nassigned to the `this` pointer as control is transferred to the method.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:695
_object_215.proto_id(55);
// CPG_CONSTRUCTION_OP:696
let constant_216=builder.constant(Constant{name:"STATIC_DISPATCH".to_owned(),value:"STATIC_DISPATCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("For statically dispatched calls the call target is known before program execution".to_owned()),schema_info:Some(info_9.clone())});
// CPG_CONSTRUCTION_OP:697
constant_216.proto_id(1);
// CPG_CONSTRUCTION_OP:698
let constant_217=builder.constant(Constant{name:"DYNAMIC_DISPATCH".to_owned(),value:"DYNAMIC_DISPATCH".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("For dynamically dispatched calls the target is determined during runtime".to_owned()),schema_info:Some(info_9.clone())});
// CPG_CONSTRUCTION_OP:699
constant_217.proto_id(2);
// CPG_CONSTRUCTION_OP:700
let constant_218=builder.constant(Constant{name:"INLINED".to_owned(),value:"INLINED".to_owned(),value_type:ValueType::String,proto_id:None,comment:Some("For macro expansions, code is inlined.".to_owned()),schema_info:Some(info_9.clone())});
// CPG_CONSTRUCTION_OP:701
constant_218.proto_id(3);
// CPG_CONSTRUCTION_OP:702
builder.add_constants("DispatchTypes".to_owned(),[constant_216.clone(),constant_217.clone(),constant_218.clone(),]);
// CPG_CONSTRUCTION_OP:703
let _object_219=builder.add_property("STATIC_RECEIVER".to_owned(),ValueType::String,"The `STATIC_RECEIVER` field is used to keep track of the type on which a static method\nis called for static methods which may be inherited. This information can then be used to find\nthe true `METHOD_FULL_NAME` of the method being called during call linking. For example, if a\nclass `Foo` defines a static method `foo` and a class `Bar extends Foo`, then the `STATIC_RECEIVER`\nof a`Bar.foo()` call is `Bar` and the `METHOD_FULL_NAME` of the `foo` call is rewritten to\n`Foo.foo:<signature>`.",Some(info_9.clone()));
// CPG_CONSTRUCTION_OP:704
_object_219.proto_id(59);
// CPG_CONSTRUCTION_OP:705
_object_202.add_property(&_object_212);
// CPG_CONSTRUCTION_OP:706
_object_202.add_property(&_object_219);
// CPG_CONSTRUCTION_OP:707
_object_202.add_out_edge(&_object_213,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:708
_object_202.add_property(&_object_207);
// CPG_CONSTRUCTION_OP:709
_object_202.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:710
_object_166.add_property(&_object_204);
// CPG_CONSTRUCTION_OP:711
_object_168.add_property(&_object_207);
// CPG_CONSTRUCTION_OP:712
_object_202.add_out_edge(&_object_215,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:713
_object_202.add_out_edge(&_object_215,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:714
_object_202.add_out_edge(&_object_215,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:715
_object_202.add_out_edge(&_object_215,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:716
_object_202.add_out_edge(&_object_215,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:717
_object_202.add_out_edge(&_object_215,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:718
_object_202.add_out_edge(&_object_215,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:719
_object_202.add_out_edge(&_object_215,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:720
_object_202.add_out_edge(&_object_214,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:721
_object_202.add_out_edge(&_object_214,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:722
_object_202.add_out_edge(&_object_214,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:723
_object_202.add_out_edge(&_object_214,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:724
_object_202.add_out_edge(&_object_214,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:725
_object_202.add_out_edge(&_object_214,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:726
_object_202.add_out_edge(&_object_214,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:727
_object_202.add_out_edge(&_object_214,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:728
_object_202.add_out_edge(&_object_214,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:729
_object_202.add_out_edge(&_object_214,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:730
_object_170.add_out_edge(&_object_214,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:731
_object_170.add_out_edge(&_object_214,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:732
_object_170.add_out_edge(&_object_214,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:733
_object_170.add_out_edge(&_object_214,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:734
_object_170.add_out_edge(&_object_214,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:735
_object_170.add_out_edge(&_object_214,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:736
_object_170.add_out_edge(&_object_214,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:737
_object_170.add_out_edge(&_object_214,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:738
_object_170.add_out_edge(&_object_214,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:739
_object_170.add_out_edge(&_object_214,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{argument_index:_object_204,argument_name:_object_205,argument_label:_object_206,method_full_name:_object_207,evaluation_strategy:_object_208,evaluation_strategies:vec![constant_209.clone(),constant_210.clone(),constant_211.clone(),],dispatch_type:_object_212,call:_object_213,argument:_object_214,receiver:_object_215,dispatch_types:vec![constant_216.clone(),constant_217.clone(),constant_218.clone(),],static_receiver:_object_219,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_method_schema:&MethodSchema,_arg_ast_schema:&AstSchema,)->Result<Self>{Self::new(builder,_arg_method_schema,_arg_ast_schema,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.CallGraph$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.CallGraph".to_owned(),schema_name:"CallGraph".to_owned(), doc_index:8,description:"\nThe Call Graph Layer represents call relations between methods.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for CallGraphSchema { fn doc_index(&self)->i32{8} fn description(&self)->&str{"\nThe Call Graph Layer represents call relations between methods.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:CallGraph
// CPG_SCHEMA_MODULE:Cfg
#[derive(Clone,Debug)] pub struct CfgSchema {pub cfg_node:NodeRef,pub cfg:EdgeRef,}impl CfgSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_method_schema:&MethodSchema,_arg_ast:&AstSchema,)->Result<Self>{let _object_138=_arg_method_schema.method.clone();let _object_140=_arg_method_schema.method_parameter_in.clone();let _object_141=_arg_method_schema.method_parameter_out.clone();let _object_142=_arg_method_schema.method_return.clone();let _object_144=_arg_ast.ast_node.clone();let _object_145=_arg_ast.block.clone();let _object_146=_arg_ast.literal.clone();let _object_148=_arg_ast.identifier.clone();let _object_150=_arg_ast.field_identifier.clone();let _object_166=_arg_ast.jump_target.clone();let _object_168=_arg_ast.method_ref.clone();let _object_169=_arg_ast.type_ref.clone();let _object_170=_arg_ast.ret.clone();let _object_187=_arg_ast.control_structure.clone();let _object_188=_arg_ast.unknown.clone();let _object_201=_arg_ast.call_repr.clone();let _object_202=_arg_ast.call_node.clone();let _object_203=_arg_ast.expression.clone();let info_10=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Cfg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Cfg".to_owned(),schema_name:"Cfg".to_owned(), doc_index:9,description:"\nThe Control Flow Graph Layer provides control flow graphs for each method. Control\nflow graphs are constructed by marking a sub set of the abstract syntax tree nodes\nas control flow nodes (`CFG_NODE`) and connecting these nodes via `CFG` edges.\nThe control flow graph models both the control flow within the calculation of an\nexpression as well as from expression to expression. The layer can be automatically\ngenerated from the syntax tree layer if only control structure types supported by\nthis specification are employed.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:740
let _object_220=builder.add_node_base_type("CFG_NODE".to_owned(),"This is the base class for all control flow nodes. It is itself\na child class of `AST_NODE`, that is, all control flow graph nodes\nare also syntax tree nodes in the CPG specification.",Some(info_10.clone()));
// CPG_CONSTRUCTION_OP:741
_object_220.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:742
_object_138.extend(&_object_220)?;
// CPG_CONSTRUCTION_OP:743
_object_142.extend(&_object_220)?;
// CPG_CONSTRUCTION_OP:744
_object_140.extend(&_object_220)?;
// CPG_CONSTRUCTION_OP:745
_object_141.extend(&_object_220)?;
// CPG_CONSTRUCTION_OP:746
_object_203.extend(&_object_220)?;
// CPG_CONSTRUCTION_OP:747
_object_201.extend(&_object_220)?;
// CPG_CONSTRUCTION_OP:748
_object_166.extend(&_object_220)?;
// CPG_CONSTRUCTION_OP:749
let _object_221=builder.add_edge_type("CFG".to_owned(),"This edge indicates control flow from the source to the destination node.",Some(info_10.clone()));
// CPG_CONSTRUCTION_OP:750
_object_221.proto_id(19);
// CPG_CONSTRUCTION_OP:751
_object_138.add_out_edge(&_object_221,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:752
_object_138.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"cfgFirst".to_owned(),out_doc:"First control flow graph node".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:753
_object_150.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:754
_object_150.add_out_edge(&_object_221,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::ZeroOrOne, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:755
_object_145.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:756
_object_202.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:757
_object_187.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:758
_object_166.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:759
_object_148.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:760
_object_146.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:761
_object_168.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:762
_object_169.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:763
_object_188.add_out_edge(&_object_221,&_object_220,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:764
_object_170.add_out_edge(&_object_221,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"toReturn".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:765
_object_202.add_out_edge(&_object_221,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:766
_object_168.add_out_edge(&_object_221,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:767
_object_169.add_out_edge(&_object_221,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{cfg_node:_object_220,cfg:_object_221,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_method_schema:&MethodSchema,_arg_ast:&AstSchema,)->Result<Self>{Self::new(builder,_arg_method_schema,_arg_ast,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Cfg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Cfg".to_owned(),schema_name:"Cfg".to_owned(), doc_index:9,description:"\nThe Control Flow Graph Layer provides control flow graphs for each method. Control\nflow graphs are constructed by marking a sub set of the abstract syntax tree nodes\nas control flow nodes (`CFG_NODE`) and connecting these nodes via `CFG` edges.\nThe control flow graph models both the control flow within the calculation of an\nexpression as well as from expression to expression. The layer can be automatically\ngenerated from the syntax tree layer if only control structure types supported by\nthis specification are employed.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for CfgSchema { fn doc_index(&self)->i32{9} fn description(&self)->&str{"\nThe Control Flow Graph Layer provides control flow graphs for each method. Control\nflow graphs are constructed by marking a sub set of the abstract syntax tree nodes\nas control flow nodes (`CFG_NODE`) and connecting these nodes via `CFG` edges.\nThe control flow graph models both the control flow within the calculation of an\nexpression as well as from expression to expression. The layer can be automatically\ngenerated from the syntax tree layer if only control structure types supported by\nthis specification are employed.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Cfg
// CPG_SCHEMA_MODULE:Dominators
#[derive(Clone,Debug)] pub struct DominatorsSchema {pub dominate:EdgeRef,pub post_dominate:EdgeRef,}impl DominatorsSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_method_schema:&MethodSchema,_arg_ast:&AstSchema,)->Result<Self>{let _object_138=_arg_method_schema.method.clone();let _object_142=_arg_method_schema.method_return.clone();let _object_145=_arg_ast.block.clone();let _object_146=_arg_ast.literal.clone();let _object_148=_arg_ast.identifier.clone();let _object_150=_arg_ast.field_identifier.clone();let _object_166=_arg_ast.jump_target.clone();let _object_168=_arg_ast.method_ref.clone();let _object_169=_arg_ast.type_ref.clone();let _object_170=_arg_ast.ret.clone();let _object_187=_arg_ast.control_structure.clone();let _object_188=_arg_ast.unknown.clone();let _object_202=_arg_ast.call_node.clone();let info_11=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Dominators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Dominators".to_owned(),schema_name:"Dominators".to_owned(), doc_index:10,description:"\nThe Dominators Layer provides dominator- and post-dominator trees for all methods.\nIt is constructed automatically from the control flow graph layer and is in turn\nused to automatically construct control dependence relations of the PDG layer.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:768
let _object_222=builder.add_edge_type("DOMINATE".to_owned(),"This edge indicates that the source node immediately dominates the destination node.",Some(info_11.clone()));
// CPG_CONSTRUCTION_OP:769
_object_222.proto_id(181);
// CPG_CONSTRUCTION_OP:770
let _object_223=builder.add_edge_type("POST_DOMINATE".to_owned(),"This edge indicates that the source node immediately post dominates the destination node.",Some(info_11.clone()));
// CPG_CONSTRUCTION_OP:771
_object_223.proto_id(182);
// CPG_CONSTRUCTION_OP:772
_object_138.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:773
_object_138.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:774
_object_138.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:775
_object_138.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:776
_object_138.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:777
_object_138.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:778
_object_138.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:779
_object_138.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:780
_object_138.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:781
_object_138.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:782
_object_142.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:783
_object_142.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:784
_object_142.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:785
_object_142.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:786
_object_142.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:787
_object_142.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:788
_object_142.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:789
_object_142.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:790
_object_142.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:791
_object_142.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:792
_object_142.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:793
_object_142.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:794
_object_146.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:795
_object_146.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:796
_object_146.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:797
_object_146.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:798
_object_146.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:799
_object_146.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:800
_object_146.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:801
_object_146.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:802
_object_146.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:803
_object_146.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:804
_object_146.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:805
_object_146.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:806
_object_146.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:807
_object_146.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:808
_object_146.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:809
_object_146.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:810
_object_146.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:811
_object_146.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:812
_object_146.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:813
_object_146.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:814
_object_146.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:815
_object_146.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:816
_object_146.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:817
_object_146.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:818
_object_202.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:819
_object_202.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:820
_object_202.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:821
_object_202.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:822
_object_202.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:823
_object_202.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:824
_object_202.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:825
_object_202.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:826
_object_202.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:827
_object_202.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:828
_object_202.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:829
_object_202.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:830
_object_202.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:831
_object_202.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:832
_object_202.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:833
_object_202.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:834
_object_202.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:835
_object_202.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:836
_object_202.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:837
_object_202.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:838
_object_202.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:839
_object_202.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:840
_object_202.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:841
_object_202.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:842
_object_148.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:843
_object_148.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:844
_object_148.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:845
_object_148.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:846
_object_148.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:847
_object_148.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:848
_object_148.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:849
_object_148.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:850
_object_148.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:851
_object_148.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:852
_object_148.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:853
_object_148.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:854
_object_148.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:855
_object_148.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:856
_object_148.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:857
_object_148.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:858
_object_148.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:859
_object_148.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:860
_object_148.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:861
_object_148.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:862
_object_148.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:863
_object_148.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:864
_object_148.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:865
_object_148.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:866
_object_150.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:867
_object_150.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:868
_object_150.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:869
_object_150.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:870
_object_150.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:871
_object_150.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:872
_object_150.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:873
_object_150.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:874
_object_150.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:875
_object_150.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:876
_object_150.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:877
_object_150.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:878
_object_150.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:879
_object_150.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:880
_object_150.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:881
_object_150.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:882
_object_150.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:883
_object_150.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:884
_object_150.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:885
_object_150.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:886
_object_150.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:887
_object_150.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:888
_object_150.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:889
_object_150.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:890
_object_170.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:891
_object_170.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:892
_object_170.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:893
_object_170.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:894
_object_170.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:895
_object_170.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:896
_object_170.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:897
_object_170.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:898
_object_170.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:899
_object_170.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:900
_object_170.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:901
_object_170.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:902
_object_170.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:903
_object_170.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:904
_object_170.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:905
_object_170.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:906
_object_170.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:907
_object_170.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:908
_object_170.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:909
_object_170.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:910
_object_170.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:911
_object_170.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:912
_object_170.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:913
_object_170.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:914
_object_145.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:915
_object_145.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:916
_object_145.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:917
_object_145.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:918
_object_145.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:919
_object_145.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:920
_object_145.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:921
_object_145.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:922
_object_145.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:923
_object_145.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:924
_object_145.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:925
_object_145.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:926
_object_145.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:927
_object_145.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:928
_object_145.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:929
_object_145.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:930
_object_145.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:931
_object_145.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:932
_object_145.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:933
_object_145.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:934
_object_145.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:935
_object_145.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:936
_object_145.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:937
_object_145.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:938
_object_188.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:939
_object_188.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:940
_object_188.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:941
_object_188.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:942
_object_188.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:943
_object_188.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:944
_object_188.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:945
_object_188.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:946
_object_188.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:947
_object_188.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:948
_object_188.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:949
_object_188.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:950
_object_188.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:951
_object_188.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:952
_object_188.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:953
_object_188.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:954
_object_188.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:955
_object_188.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:956
_object_188.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:957
_object_188.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:958
_object_188.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:959
_object_188.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:960
_object_188.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:961
_object_188.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:962
_object_187.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:963
_object_187.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:964
_object_187.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:965
_object_187.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:966
_object_187.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:967
_object_187.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:968
_object_187.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:969
_object_187.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:970
_object_187.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:971
_object_187.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:972
_object_187.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:973
_object_187.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:974
_object_187.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:975
_object_187.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:976
_object_187.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:977
_object_187.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:978
_object_187.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:979
_object_187.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:980
_object_187.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:981
_object_187.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:982
_object_187.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:983
_object_187.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:984
_object_187.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:985
_object_187.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:986
_object_168.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:987
_object_168.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:988
_object_168.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:989
_object_168.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:990
_object_168.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:991
_object_168.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:992
_object_168.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:993
_object_168.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:994
_object_168.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:995
_object_168.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:996
_object_168.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:997
_object_168.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:998
_object_168.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:999
_object_168.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1000
_object_168.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1001
_object_168.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1002
_object_168.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1003
_object_168.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1004
_object_168.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1005
_object_168.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1006
_object_168.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1007
_object_168.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1008
_object_168.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1009
_object_168.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1010
_object_169.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1011
_object_169.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1012
_object_169.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1013
_object_169.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1014
_object_169.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1015
_object_169.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1016
_object_169.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1017
_object_169.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1018
_object_169.add_out_edge(&_object_222,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1019
_object_169.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1020
_object_169.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1021
_object_169.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1022
_object_169.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1023
_object_169.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1024
_object_169.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1025
_object_169.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1026
_object_169.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1027
_object_169.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1028
_object_169.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1029
_object_169.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1030
_object_169.add_out_edge(&_object_223,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1031
_object_169.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1032
_object_169.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1033
_object_169.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1034
_object_166.add_out_edge(&_object_222,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1035
_object_166.add_out_edge(&_object_222,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1036
_object_166.add_out_edge(&_object_222,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1037
_object_166.add_out_edge(&_object_222,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1038
_object_166.add_out_edge(&_object_222,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1039
_object_166.add_out_edge(&_object_222,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1040
_object_166.add_out_edge(&_object_222,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1041
_object_166.add_out_edge(&_object_222,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1042
_object_166.add_out_edge(&_object_222,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1043
_object_166.add_out_edge(&_object_222,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1044
_object_166.add_out_edge(&_object_222,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1045
_object_166.add_out_edge(&_object_223,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1046
_object_166.add_out_edge(&_object_223,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1047
_object_166.add_out_edge(&_object_223,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1048
_object_166.add_out_edge(&_object_223,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1049
_object_166.add_out_edge(&_object_223,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1050
_object_166.add_out_edge(&_object_223,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1051
_object_166.add_out_edge(&_object_223,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1052
_object_166.add_out_edge(&_object_223,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1053
_object_166.add_out_edge(&_object_223,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1054
_object_166.add_out_edge(&_object_223,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1055
_object_166.add_out_edge(&_object_223,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{dominate:_object_222,post_dominate:_object_223,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_method_schema:&MethodSchema,_arg_ast:&AstSchema,)->Result<Self>{Self::new(builder,_arg_method_schema,_arg_ast,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Dominators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Dominators".to_owned(),schema_name:"Dominators".to_owned(), doc_index:10,description:"\nThe Dominators Layer provides dominator- and post-dominator trees for all methods.\nIt is constructed automatically from the control flow graph layer and is in turn\nused to automatically construct control dependence relations of the PDG layer.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for DominatorsSchema { fn doc_index(&self)->i32{10} fn description(&self)->&str{"\nThe Dominators Layer provides dominator- and post-dominator trees for all methods.\nIt is constructed automatically from the control flow graph layer and is in turn\nused to automatically construct control dependence relations of the PDG layer.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Dominators
// CPG_SCHEMA_MODULE:Pdg
#[derive(Clone,Debug)] pub struct PdgSchema {pub variable:PropertyRef,pub cdg:EdgeRef,pub reaching_def:EdgeRef,}impl PdgSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_method_schema:&MethodSchema,_arg_ast:&AstSchema,)->Result<Self>{let _object_138=_arg_method_schema.method.clone();let _object_140=_arg_method_schema.method_parameter_in.clone();let _object_141=_arg_method_schema.method_parameter_out.clone();let _object_142=_arg_method_schema.method_return.clone();let _object_145=_arg_ast.block.clone();let _object_146=_arg_ast.literal.clone();let _object_148=_arg_ast.identifier.clone();let _object_150=_arg_ast.field_identifier.clone();let _object_166=_arg_ast.jump_target.clone();let _object_168=_arg_ast.method_ref.clone();let _object_169=_arg_ast.type_ref.clone();let _object_170=_arg_ast.ret.clone();let _object_187=_arg_ast.control_structure.clone();let _object_188=_arg_ast.unknown.clone();let _object_202=_arg_ast.call_node.clone();let info_12=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Pdg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Pdg".to_owned(),schema_name:"Pdg".to_owned(), doc_index:11,description:"\n      The Program Dependence Graph Layer contains a program dependence graph for\neach method of the source program. A program dependence graph consists\nof a data dependence graph (DDG) and a control dependence graph (CDG),\ncreated by connecting nodes of the control flow graph via `REACHING_DEF`\nand `CDG` edges respectively.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:1056
let _object_224=builder.add_property("VARIABLE".to_owned(),ValueType::String,"This edge property represents the variable propagated by a reaching definition edge.",Some(info_12.clone()));
// CPG_CONSTRUCTION_OP:1057
_object_224.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:1058
_object_224.proto_id(11);
// CPG_CONSTRUCTION_OP:1059
let _object_225=builder.add_edge_type("CDG".to_owned(),"A CDG edge expresses that the destination node is control dependent on the source node.",Some(info_12.clone()));
// CPG_CONSTRUCTION_OP:1060
_object_225.proto_id(183);
// CPG_CONSTRUCTION_OP:1061
let _object_226=builder.add_edge_type("REACHING_DEF".to_owned(),"A reaching definition edge indicates that a variable produced at the source node reaches\nthe destination node without being reassigned on the way. The `VARIABLE` property indicates\nwhich variable is propagated.",Some(info_12.clone()));
// CPG_CONSTRUCTION_OP:1062
_object_226.proto_id(137);
// CPG_CONSTRUCTION_OP:1063
_object_226.with_property(Some(&_object_224));
// CPG_CONSTRUCTION_OP:1064
_object_146.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1065
_object_146.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1066
_object_146.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1067
_object_146.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1068
_object_146.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1069
_object_146.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1070
_object_146.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1071
_object_146.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1072
_object_146.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1073
_object_146.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1074
_object_146.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1075
_object_146.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1076
_object_202.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1077
_object_202.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1078
_object_202.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1079
_object_202.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1080
_object_202.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1081
_object_202.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1082
_object_202.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1083
_object_202.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1084
_object_202.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1085
_object_202.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1086
_object_202.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1087
_object_202.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1088
_object_148.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1089
_object_148.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1090
_object_148.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1091
_object_148.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1092
_object_148.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1093
_object_148.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1094
_object_148.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1095
_object_148.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1096
_object_148.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1097
_object_148.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1098
_object_148.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1099
_object_148.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1100
_object_150.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1101
_object_150.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1102
_object_150.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1103
_object_150.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1104
_object_150.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1105
_object_150.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1106
_object_150.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1107
_object_150.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1108
_object_150.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1109
_object_150.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1110
_object_150.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1111
_object_150.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1112
_object_145.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1113
_object_145.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1114
_object_145.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1115
_object_145.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1116
_object_145.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1117
_object_145.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1118
_object_145.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1119
_object_145.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1120
_object_145.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1121
_object_145.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1122
_object_145.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1123
_object_145.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1124
_object_188.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1125
_object_188.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1126
_object_188.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1127
_object_188.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1128
_object_188.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1129
_object_188.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1130
_object_188.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1131
_object_188.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1132
_object_188.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1133
_object_188.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1134
_object_188.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1135
_object_188.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1136
_object_187.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1137
_object_187.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1138
_object_187.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1139
_object_187.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1140
_object_187.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1141
_object_187.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1142
_object_187.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1143
_object_187.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1144
_object_187.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1145
_object_187.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1146
_object_187.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1147
_object_187.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1148
_object_168.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1149
_object_168.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1150
_object_168.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1151
_object_168.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1152
_object_168.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1153
_object_168.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1154
_object_168.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1155
_object_168.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1156
_object_168.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1157
_object_168.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1158
_object_168.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1159
_object_168.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1160
_object_169.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1161
_object_169.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1162
_object_169.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1163
_object_169.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1164
_object_169.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1165
_object_169.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1166
_object_169.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1167
_object_169.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1168
_object_169.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1169
_object_169.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1170
_object_169.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1171
_object_169.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1172
_object_166.add_out_edge(&_object_225,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1173
_object_166.add_out_edge(&_object_225,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1174
_object_166.add_out_edge(&_object_225,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1175
_object_166.add_out_edge(&_object_225,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1176
_object_166.add_out_edge(&_object_225,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1177
_object_166.add_out_edge(&_object_225,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1178
_object_166.add_out_edge(&_object_225,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1179
_object_166.add_out_edge(&_object_225,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1180
_object_166.add_out_edge(&_object_225,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1181
_object_166.add_out_edge(&_object_225,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1182
_object_166.add_out_edge(&_object_225,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1183
_object_166.add_out_edge(&_object_225,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1184
_object_138.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1185
_object_138.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1186
_object_138.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1187
_object_138.add_out_edge(&_object_226,&_object_140,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1188
_object_138.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1189
_object_138.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1190
_object_138.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1191
_object_138.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1192
_object_170.add_out_edge(&_object_226,&_object_142,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1193
_object_170.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1194
_object_170.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1195
_object_170.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1196
_object_170.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1197
_object_168.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1198
_object_168.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1199
_object_168.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1200
_object_168.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1201
_object_168.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1202
_object_168.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1203
_object_168.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1204
_object_169.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1205
_object_169.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1206
_object_169.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1207
_object_169.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1208
_object_169.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1209
_object_169.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1210
_object_169.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1211
_object_140.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1212
_object_140.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1213
_object_140.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1214
_object_140.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1215
_object_140.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1216
_object_140.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1217
_object_140.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1218
_object_141.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1219
_object_141.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1220
_object_141.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1221
_object_141.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1222
_object_141.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1223
_object_141.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1224
_object_146.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1225
_object_146.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1226
_object_146.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1227
_object_146.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1228
_object_146.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1229
_object_146.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1230
_object_146.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1231
_object_202.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1232
_object_202.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1233
_object_202.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1234
_object_202.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1235
_object_202.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1236
_object_202.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1237
_object_202.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1238
_object_148.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1239
_object_148.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1240
_object_148.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1241
_object_148.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1242
_object_148.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1243
_object_148.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1244
_object_148.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1245
_object_145.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1246
_object_145.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1247
_object_145.add_out_edge(&_object_226,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1248
_object_145.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1249
_object_145.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1250
_object_145.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1251
_object_145.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1252
_object_145.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1253
_object_187.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1254
_object_187.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1255
_object_187.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1256
_object_187.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1257
_object_187.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1258
_object_187.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1259
_object_187.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1260
_object_188.add_out_edge(&_object_226,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1261
_object_188.add_out_edge(&_object_226,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1262
_object_188.add_out_edge(&_object_226,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1263
_object_188.add_out_edge(&_object_226,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1264
_object_188.add_out_edge(&_object_226,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1265
_object_188.add_out_edge(&_object_226,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1266
_object_188.add_out_edge(&_object_226,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{variable:_object_224,cdg:_object_225,reaching_def:_object_226,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_method_schema:&MethodSchema,_arg_ast:&AstSchema,)->Result<Self>{Self::new(builder,_arg_method_schema,_arg_ast,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Pdg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Pdg".to_owned(),schema_name:"Pdg".to_owned(), doc_index:11,description:"\n      The Program Dependence Graph Layer contains a program dependence graph for\neach method of the source program. A program dependence graph consists\nof a data dependence graph (DDG) and a control dependence graph (CDG),\ncreated by connecting nodes of the control flow graph via `REACHING_DEF`\nand `CDG` edges respectively.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for PdgSchema { fn doc_index(&self)->i32{11} fn description(&self)->&str{"\n      The Program Dependence Graph Layer contains a program dependence graph for\neach method of the source program. A program dependence graph consists\nof a data dependence graph (DDG) and a control dependence graph (CDG),\ncreated by connecting nodes of the control flow graph via `REACHING_DEF`\nand `CDG` edges respectively.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Pdg
// CPG_SCHEMA_MODULE:Shortcuts
#[derive(Clone,Debug)] pub struct ShortcutsSchema {pub eval_type:EdgeRef,pub contains:EdgeRef,pub parameter_link:EdgeRef,}impl ShortcutsSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_method_schema:&MethodSchema,_arg_ast_schema:&AstSchema,_arg_type_schema:&TypeSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{let _object_129=_arg_type_schema.type_decl.clone();let _object_132=_arg_type_schema.member.clone();let _object_133=_arg_type_schema.tpe.clone();let _object_138=_arg_method_schema.method.clone();let _object_14=_arg_base.r#ref.clone();let _object_140=_arg_method_schema.method_parameter_in.clone();let _object_141=_arg_method_schema.method_parameter_out.clone();let _object_142=_arg_method_schema.method_return.clone();let _object_145=_arg_ast_schema.block.clone();let _object_146=_arg_ast_schema.literal.clone();let _object_147=_arg_ast_schema.local.clone();let _object_148=_arg_ast_schema.identifier.clone();let _object_150=_arg_ast_schema.field_identifier.clone();let _object_166=_arg_ast_schema.jump_target.clone();let _object_168=_arg_ast_schema.method_ref.clone();let _object_169=_arg_ast_schema.type_ref.clone();let _object_170=_arg_ast_schema.ret.clone();let _object_187=_arg_ast_schema.control_structure.clone();let _object_188=_arg_ast_schema.unknown.clone();let _object_202=_arg_ast_schema.call_node.clone();let _object_21=_arg_fs.file.clone();let info_13=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Shortcuts$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Shortcuts".to_owned(),schema_name:"Shortcuts".to_owned(), doc_index:16,description:"\nThe Shortcuts Layer provides shortcut edges calculated to speed up\nsubsequent queries. Language frontends MUST NOT create shortcut edges.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:1267
let _object_227=builder.add_edge_type("EVAL_TYPE".to_owned(),"This edge connects a node to its evaluation type.",Some(info_13.clone()));
// CPG_CONSTRUCTION_OP:1268
_object_227.proto_id(21);
// CPG_CONSTRUCTION_OP:1269
let _object_228=builder.add_edge_type("CONTAINS".to_owned(),"This edge connects a node to the method that contains it.",Some(info_13.clone()));
// CPG_CONSTRUCTION_OP:1270
_object_228.proto_id(28);
// CPG_CONSTRUCTION_OP:1271
let _object_229=builder.add_edge_type("PARAMETER_LINK".to_owned(),"This edge connects a method input parameter to the corresponding\nmethod output parameter.",Some(info_13.clone()));
// CPG_CONSTRUCTION_OP:1272
_object_229.proto_id(12);
// CPG_CONSTRUCTION_OP:1273
_object_140.add_out_edge(&_object_229,&_object_141,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"asOutput".to_owned(),out_doc:"Traverse to corresponding formal output parameter".to_owned(),in_step:"asInput".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1274
_object_21.add_out_edge(&_object_228,&_object_129,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1275
_object_21.add_out_edge(&_object_228,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1276
_object_138.add_out_edge(&_object_228,&_object_202,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1277
_object_138.add_out_edge(&_object_228,&_object_148,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1278
_object_138.add_out_edge(&_object_228,&_object_150,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1279
_object_138.add_out_edge(&_object_228,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"literal".to_owned(),out_doc:"Literals used in the method".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1280
_object_138.add_out_edge(&_object_228,&_object_170,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1281
_object_138.add_out_edge(&_object_228,&_object_168,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1282
_object_138.add_out_edge(&_object_228,&_object_169,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1283
_object_138.add_out_edge(&_object_228,&_object_145,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1284
_object_138.add_out_edge(&_object_228,&_object_187,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1285
_object_138.add_out_edge(&_object_228,&_object_166,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1286
_object_138.add_out_edge(&_object_228,&_object_188,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1287
_object_140.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::List, out_step:"typ".to_owned(),out_doc:"Traverse to parameter type".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1288
_object_141.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"typ".to_owned(),out_doc:"Traverse to parameter type".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1289
_object_142.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1290
_object_168.add_out_edge(&_object_14,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::List, out_step:"referencedMethod".to_owned(),out_doc:"Traverse to referenced method.".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1291
_object_168.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1292
_object_169.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1293
_object_133.add_out_edge(&_object_14,&_object_129,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"referencedTypeDecl".to_owned(),out_doc:"Type declaration which is referenced by this type.".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1294
_object_129.add_out_edge(&_object_228,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1295
_object_132.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"typ".to_owned(),out_doc:"Traverse to member type".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1296
_object_146.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1297
_object_202.add_out_edge(&_object_14,&_object_132,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"referencedMember".to_owned(),out_doc:"Traverse to referenced members".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1298
_object_202.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1299
_object_147.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"typ".to_owned(),out_doc:"The type of the local.".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1300
_object_148.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1301
_object_145.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1302
_object_187.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1303
_object_188.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{eval_type:_object_227,contains:_object_228,parameter_link:_object_229,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_method_schema:&MethodSchema,_arg_ast_schema:&AstSchema,_arg_type_schema:&TypeSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{Self::new(builder,_arg_base,_arg_method_schema,_arg_ast_schema,_arg_type_schema,_arg_fs,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Shortcuts$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Shortcuts".to_owned(),schema_name:"Shortcuts".to_owned(), doc_index:16,description:"\nThe Shortcuts Layer provides shortcut edges calculated to speed up\nsubsequent queries. Language frontends MUST NOT create shortcut edges.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for ShortcutsSchema { fn doc_index(&self)->i32{16} fn description(&self)->&str{"\nThe Shortcuts Layer provides shortcut edges calculated to speed up\nsubsequent queries. Language frontends MUST NOT create shortcut edges.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Shortcuts
// CPG_SCHEMA_MODULE:Comment
#[derive(Clone,Debug)] pub struct CommentSchema {pub comment:NodeRef,}impl CommentSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base_schema:&BaseSchema,_arg_ast_schema:&AstSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{let _object_144=_arg_ast_schema.ast_node.clone();let _object_15=_arg_fs.filename.clone();let _object_189=_arg_ast_schema.ast.clone();let _object_20=_arg_fs.source_file.clone();let _object_21=_arg_fs.file.clone();let _object_3=_arg_base_schema.code.clone();let info_14=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Comment$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Comment".to_owned(),schema_name:"Comment".to_owned(), doc_index:12,description:"".to_owned(),provided_by_frontend:true};// CPG_CONSTRUCTION_OP:1304
let _object_230=builder.add_node_type("COMMENT".to_owned(),"A source code comment",Some(info_14.clone()));
// CPG_CONSTRUCTION_OP:1305
_object_230.proto_id(511);
// CPG_CONSTRUCTION_OP:1306
_object_230.add_property(&_object_15);
// CPG_CONSTRUCTION_OP:1307
_object_230.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:1308
_object_230.primary_key(Some(&_object_3));
// CPG_CONSTRUCTION_OP:1309
_object_230.add_out_edge(&_object_20,&_object_230,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"file".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1310
_object_21.add_out_edge(&_object_189,&_object_230,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"comment".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{comment:_object_230,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base_schema:&BaseSchema,_arg_ast_schema:&AstSchema,_arg_fs:&FileSystemSchema,)->Result<Self>{Self::new(builder,_arg_base_schema,_arg_ast_schema,_arg_fs,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Comment$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Comment".to_owned(),schema_name:"Comment".to_owned(), doc_index:12,description:"".to_owned(),provided_by_frontend:true}} } impl SchemaBase for CommentSchema { fn doc_index(&self)->i32{12} fn description(&self)->&str{""} fn provided_by_frontend(&self)->bool{true} }// CPG_SCHEMA_MODULE_END:Comment
// CPG_SCHEMA_MODULE:Tags
#[derive(Clone,Debug)] pub struct TagsSchema {pub tagged_by:EdgeRef,pub tag:NodeRef,pub tag_node_pair:NodeRef,}impl TagsSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_type_schema:&TypeSchema,_arg_method_schema:&MethodSchema,_arg_ast:&AstSchema,_arg_fs:&FileSystemSchema,_arg_call_graph:&CallGraphSchema,)->Result<Self>{let _object_129=_arg_type_schema.type_decl.clone();let _object_132=_arg_type_schema.member.clone();let _object_138=_arg_method_schema.method.clone();let _object_140=_arg_method_schema.method_parameter_in.clone();let _object_141=_arg_method_schema.method_parameter_out.clone();let _object_142=_arg_method_schema.method_return.clone();let _object_145=_arg_ast.block.clone();let _object_146=_arg_ast.literal.clone();let _object_147=_arg_ast.local.clone();let _object_148=_arg_ast.identifier.clone();let _object_150=_arg_ast.field_identifier.clone();let _object_166=_arg_ast.jump_target.clone();let _object_168=_arg_ast.method_ref.clone();let _object_169=_arg_ast.type_ref.clone();let _object_170=_arg_ast.ret.clone();let _object_187=_arg_ast.control_structure.clone();let _object_188=_arg_ast.unknown.clone();let _object_202=_arg_ast.call_node.clone();let _object_21=_arg_fs.file.clone();let _object_6=_arg_base.name.clone();let _object_9=_arg_base.value.clone();let info_15=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Tags$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Tags".to_owned(),schema_name:"Tags".to_owned(), doc_index:17,description:"\nThe Code Property Graph specification allows for tags to be attached\nto arbitrary nodes. Conceptually, this is similar to the creation of\nFinding nodes, however, tags are to be used for intermediate results\nrather than end-results that are to be reported to the user.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:1311
let _object_231=builder.add_edge_type("TAGGED_BY".to_owned(),"Edges from nodes to the tags they are tagged by.",Some(info_15.clone()));
// CPG_CONSTRUCTION_OP:1312
_object_231.proto_id(11);
// CPG_CONSTRUCTION_OP:1313
let _object_232=builder.add_node_type("TAG".to_owned(),"This node represents a tag.",Some(info_15.clone()));
// CPG_CONSTRUCTION_OP:1314
_object_232.proto_id(24);
// CPG_CONSTRUCTION_OP:1315
_object_232.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:1316
_object_232.add_property(&_object_9);
// CPG_CONSTRUCTION_OP:1317
_object_232.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:1318
let _object_233=builder.add_node_type("TAG_NODE_PAIR".to_owned(),"This node contains an arbitrary node and an associated tag node.",Some(info_15.clone()));
// CPG_CONSTRUCTION_OP:1319
_object_233.proto_id(208);
// CPG_CONSTRUCTION_OP:1320
_object_233.add_contained_node(&_object_232,"tag".to_owned(),Cardinality::One { default_wrapper_is_null:false,default:Some(Scalar::Null) },"");
// CPG_CONSTRUCTION_OP:1321
let _object_234=builder.any_node();
// CPG_CONSTRUCTION_OP:1322
_object_233.add_contained_node(&_object_234,"node".to_owned(),Cardinality::One { default_wrapper_is_null:false,default:Some(Scalar::Null) },"");
// CPG_CONSTRUCTION_OP:1323
_object_138.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1324
_object_142.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1325
_object_146.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1326
_object_147.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1327
_object_132.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1328
_object_202.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1329
_object_148.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1330
_object_150.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1331
_object_140.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1332
_object_170.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1333
_object_145.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1334
_object_188.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1335
_object_187.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1336
_object_168.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1337
_object_169.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1338
_object_166.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1339
_object_21.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1340
_object_141.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1341
_object_232.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1342
_object_129.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{tagged_by:_object_231,tag:_object_232,tag_node_pair:_object_233,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_type_schema:&TypeSchema,_arg_method_schema:&MethodSchema,_arg_ast:&AstSchema,_arg_fs:&FileSystemSchema,_arg_call_graph:&CallGraphSchema,)->Result<Self>{Self::new(builder,_arg_base,_arg_type_schema,_arg_method_schema,_arg_ast,_arg_fs,_arg_call_graph,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Tags$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Tags".to_owned(),schema_name:"Tags".to_owned(), doc_index:17,description:"\nThe Code Property Graph specification allows for tags to be attached\nto arbitrary nodes. Conceptually, this is similar to the creation of\nFinding nodes, however, tags are to be used for intermediate results\nrather than end-results that are to be reported to the user.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for TagsSchema { fn doc_index(&self)->i32{17} fn description(&self)->&str{"\nThe Code Property Graph specification allows for tags to be attached\nto arbitrary nodes. Conceptually, this is similar to the creation of\nFinding nodes, however, tags are to be used for intermediate results\nrather than end-results that are to be reported to the user.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Tags
// CPG_SCHEMA_MODULE:Binding
#[derive(Clone,Debug)] pub struct BindingSchema {pub binding:NodeRef,pub binds:EdgeRef,}impl BindingSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_type_decl_schema:&TypeSchema,_arg_method_schema:&MethodSchema,_arg_call_graph_schema:&CallGraphSchema,)->Result<Self>{let _object_129=_arg_type_decl_schema.type_decl.clone();let _object_137=_arg_method_schema.signature.clone();let _object_138=_arg_method_schema.method.clone();let _object_14=_arg_base.r#ref.clone();let _object_207=_arg_call_graph_schema.method_full_name.clone();let _object_6=_arg_base.name.clone();let info_16=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Binding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Binding".to_owned(),schema_name:"Binding".to_owned(), doc_index:19,description:"\nWe use the concept of \"bindings\" to support resolving of\n(method-name, signature) pairs at type declarations (`TYPE_DECL`). For each\npair that we can resolve, we create a `BINDING` node that is connected the\nthe type declaration via an incoming `BINDS` edge. The `BINDING` node is\nconnected to the method it resolves to via an outgoing `REF` edge.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:1343
let _object_235=builder.add_node_type("BINDING".to_owned(),"`BINDING` nodes represent name-signature pairs that can be resolved at a\ntype declaration (`TYPE_DECL`). They are connected to `TYPE_DECL` nodes via\nincoming `BINDS` edges. The bound method is either associated with an outgoing\n`REF` edge to a `METHOD` or with the `METHOD_FULL_NAME` property. The `REF` edge\nif present has priority.",Some(info_16.clone()));
// CPG_CONSTRUCTION_OP:1344
_object_235.proto_id(146);
// CPG_CONSTRUCTION_OP:1345
_object_235.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:1346
_object_235.add_property(&_object_137);
// CPG_CONSTRUCTION_OP:1347
_object_235.add_property(&_object_207);
// CPG_CONSTRUCTION_OP:1348
let _object_236=builder.add_edge_type("BINDS".to_owned(),"This edge connects a type declaration (`TYPE_DECL`) with a binding node (`BINDING`) and\nindicates that the type declaration has the binding represented by the binding node, in\nother words, there is a (name, signature) pair that can be resolved for the type\ndeclaration as stored in the binding node.",Some(info_16.clone()));
// CPG_CONSTRUCTION_OP:1349
_object_236.proto_id(155);
// CPG_CONSTRUCTION_OP:1350
_object_129.add_out_edge(&_object_236,&_object_235,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::One, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"bindingTypeDecl".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1351
_object_235.add_out_edge(&_object_14,&_object_138,EdgeConnection{out_cardinality:EdgeCardinality::One,in_cardinality:EdgeCardinality::List, out_step:"boundMethod".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{binding:_object_235,binds:_object_236,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_type_decl_schema:&TypeSchema,_arg_method_schema:&MethodSchema,_arg_call_graph_schema:&CallGraphSchema,)->Result<Self>{Self::new(builder,_arg_base,_arg_type_decl_schema,_arg_method_schema,_arg_call_graph_schema,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Binding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Binding".to_owned(),schema_name:"Binding".to_owned(), doc_index:19,description:"\nWe use the concept of \"bindings\" to support resolving of\n(method-name, signature) pairs at type declarations (`TYPE_DECL`). For each\npair that we can resolve, we create a `BINDING` node that is connected the\nthe type declaration via an incoming `BINDS` edge. The `BINDING` node is\nconnected to the method it resolves to via an outgoing `REF` edge.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for BindingSchema { fn doc_index(&self)->i32{19} fn description(&self)->&str{"\nWe use the concept of \"bindings\" to support resolving of\n(method-name, signature) pairs at type declarations (`TYPE_DECL`). For each\npair that we can resolve, we create a `BINDING` node that is connected the\nthe type declaration via an incoming `BINDS` edge. The `BINDING` node is\nconnected to the method it resolves to via an outgoing `REF` edge.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Binding
// CPG_SCHEMA_MODULE:Annotation
#[derive(Clone,Debug)] pub struct AnnotationSchema {pub annotation:NodeRef,pub annotation_parameter_assign:NodeRef,pub annotation_parameter:NodeRef,pub annotation_literal:NodeRef,pub array_initializer:NodeRef,}impl AnnotationSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_method_schema:&MethodSchema,_arg_type_schema:&TypeSchema,_arg_ast_schema:&AstSchema,_arg_shortcuts:&ShortcutsSchema,)->Result<Self>{let _object_129=_arg_type_schema.type_decl.clone();let _object_132=_arg_type_schema.member.clone();let _object_133=_arg_type_schema.tpe.clone();let _object_138=_arg_method_schema.method.clone();let _object_140=_arg_method_schema.method_parameter_in.clone();let _object_144=_arg_ast_schema.ast_node.clone();let _object_146=_arg_ast_schema.literal.clone();let _object_148=_arg_ast_schema.identifier.clone();let _object_168=_arg_ast_schema.method_ref.clone();let _object_188=_arg_ast_schema.unknown.clone();let _object_189=_arg_ast_schema.ast.clone();let _object_203=_arg_ast_schema.expression.clone();let _object_227=_arg_shortcuts.eval_type.clone();let _object_6=_arg_base.name.clone();let _object_7=_arg_base.full_name.clone();let info_17=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Annotation$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Annotation".to_owned(),schema_name:"Annotation".to_owned(), doc_index:20,description:"\nJava Annotation related CPG definitions.\n".to_owned(),provided_by_frontend:true};// CPG_CONSTRUCTION_OP:1352
let _object_237=builder.add_node_type("ANNOTATION".to_owned(),"A method annotation.\nThe semantics of the FULL_NAME property on this node differ from the usual FULL_NAME\nsemantics in the sense that FULL_NAME describes the represented annotation class/interface\nitself and not the ANNOTATION node.",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1353
_object_237.proto_id(5);
// CPG_CONSTRUCTION_OP:1354
_object_237.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:1355
_object_237.add_property(&_object_7);
// CPG_CONSTRUCTION_OP:1356
_object_237.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:1357
let _object_238=builder.add_node_type("ANNOTATION_PARAMETER_ASSIGN".to_owned(),"Assignment of annotation argument to annotation parameter",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1358
_object_238.proto_id(6);
// CPG_CONSTRUCTION_OP:1359
_object_238.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:1360
let _object_239=builder.add_node_type("ANNOTATION_PARAMETER".to_owned(),"Formal annotation parameter",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1361
_object_239.proto_id(7);
// CPG_CONSTRUCTION_OP:1362
_object_239.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:1363
let _object_240=builder.add_node_type("ANNOTATION_LITERAL".to_owned(),"A literal value assigned to an ANNOTATION_PARAMETER",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1364
_object_240.proto_id(49);
// CPG_CONSTRUCTION_OP:1365
_object_240.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:1366
_object_240.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:1367
let _object_241=builder.add_node_type("ARRAY_INITIALIZER".to_owned(),"Initialization construct for arrays",Some(info_17.clone()));
// CPG_CONSTRUCTION_OP:1368
_object_241.proto_id(14);
// CPG_CONSTRUCTION_OP:1369
_object_241.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:1370
_object_241.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:1371
_object_237.add_out_edge(&_object_189,&_object_238,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1372
_object_238.add_out_edge(&_object_189,&_object_239,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1373
_object_238.add_out_edge(&_object_189,&_object_241,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1374
_object_238.add_out_edge(&_object_189,&_object_240,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1375
_object_238.add_out_edge(&_object_189,&_object_237,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1376
_object_241.add_out_edge(&_object_189,&_object_146,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1377
_object_241.add_out_edge(&_object_227,&_object_133,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1378
_object_146.add_out_edge(&_object_189,&_object_237,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1379
_object_148.add_out_edge(&_object_189,&_object_237,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1380
_object_138.add_out_edge(&_object_189,&_object_237,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1381
_object_140.add_out_edge(&_object_189,&_object_237,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1382
_object_168.add_out_edge(&_object_189,&_object_237,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1383
_object_129.add_out_edge(&_object_189,&_object_237,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1384
_object_132.add_out_edge(&_object_189,&_object_237,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1385
_object_188.add_out_edge(&_object_189,&_object_237,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
 Ok(Self{annotation:_object_237,annotation_parameter_assign:_object_238,annotation_parameter:_object_239,annotation_literal:_object_240,array_initializer:_object_241,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_method_schema:&MethodSchema,_arg_type_schema:&TypeSchema,_arg_ast_schema:&AstSchema,_arg_shortcuts:&ShortcutsSchema,)->Result<Self>{Self::new(builder,_arg_base,_arg_method_schema,_arg_type_schema,_arg_ast_schema,_arg_shortcuts,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Annotation$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Annotation".to_owned(),schema_name:"Annotation".to_owned(), doc_index:20,description:"\nJava Annotation related CPG definitions.\n".to_owned(),provided_by_frontend:true}} } impl SchemaBase for AnnotationSchema { fn doc_index(&self)->i32{20} fn description(&self)->&str{"\nJava Annotation related CPG definitions.\n"} fn provided_by_frontend(&self)->bool{true} }// CPG_SCHEMA_MODULE_END:Annotation
// CPG_SCHEMA_MODULE:Finding
#[derive(Clone,Debug)] pub struct FindingSchema {pub key:PropertyRef,pub finding:NodeRef,pub key_value_pair:NodeRef,pub evidence_description:PropertyRef,}impl FindingSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,)->Result<Self>{let _object_234=builder.any_node();let _object_9=_arg_base.value.clone();let info_18=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Finding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Finding".to_owned(),schema_name:"Finding".to_owned(), doc_index:15,description:"\nWe allow findings (e.g., potential vulnerabilities, notes on dangerous practices) to\nbe stored in the Findings Layer.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:1386
let _object_242=builder.add_property("KEY".to_owned(),ValueType::String,"This property denotes a key of a key-value pair.",Some(info_18.clone()));
// CPG_CONSTRUCTION_OP:1387
_object_242.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:1388
_object_242.proto_id(131);
// CPG_CONSTRUCTION_OP:1389
let _object_243=builder.add_node_type("FINDING".to_owned(),"Finding nodes may be used to store analysis results in the graph\nthat are to be exposed to an end-user, e.g., information about\npotential vulnerabilities or dangerous programming practices.\nA Finding node may contain an abitrary list of key value pairs\nthat characterize the finding, as well as a list of nodes that\nserve as evidence for the finding.",Some(info_18.clone()));
// CPG_CONSTRUCTION_OP:1390
_object_243.proto_id(214);
// CPG_CONSTRUCTION_OP:1391
let _object_244=builder.add_node_type("KEY_VALUE_PAIR".to_owned(),"This node represents a key value pair, where both the key and the value are strings.",Some(info_18.clone()));
// CPG_CONSTRUCTION_OP:1392
_object_244.proto_id(217);
// CPG_CONSTRUCTION_OP:1393
_object_244.add_property(&_object_242);
// CPG_CONSTRUCTION_OP:1394
_object_244.add_property(&_object_9);
// CPG_CONSTRUCTION_OP:1395
let _object_245=builder.add_property("EVIDENCE_DESCRIPTION".to_owned(),ValueType::String,"Optional description for nodes in evidence. Used to give a hint about the kind of evidence\nprovided by a node. The evidence description and evidence nodes are associated by index.",Some(info_18.clone()));
// CPG_CONSTRUCTION_OP:1396
_object_245.as_list();
// CPG_CONSTRUCTION_OP:1397
_object_245.proto_id(2007162);
// CPG_CONSTRUCTION_OP:1398
_object_243.add_contained_node(&_object_234,"evidence".to_owned(),Cardinality::List,"");
// CPG_CONSTRUCTION_OP:1399
_object_243.add_contained_node(&_object_244,"keyValuePairs".to_owned(),Cardinality::List,"");
// CPG_CONSTRUCTION_OP:1400
_object_243.add_property(&_object_245);
 Ok(Self{key:_object_242,finding:_object_243,key_value_pair:_object_244,evidence_description:_object_245,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,)->Result<Self>{Self::new(builder,_arg_base,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Finding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Finding".to_owned(),schema_name:"Finding".to_owned(), doc_index:15,description:"\nWe allow findings (e.g., potential vulnerabilities, notes on dangerous practices) to\nbe stored in the Findings Layer.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for FindingSchema { fn doc_index(&self)->i32{15} fn description(&self)->&str{"\nWe allow findings (e.g., potential vulnerabilities, notes on dangerous practices) to\nbe stored in the Findings Layer.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Finding
// CPG_SCHEMA_MODULE:Hidden
#[derive(Clone,Debug)] pub struct HiddenSchema {pub closure_binding_id:PropertyRef,pub capture:EdgeRef,pub closure_binding:NodeRef,pub captured_by:EdgeRef,pub template_dom:NodeRef,pub dependency_group_id:PropertyRef,pub dependency:NodeRef,pub dynamic_type_hint_full_name:PropertyRef,pub possible_types:PropertyRef,pub import_node:NodeRef,pub imported_entity:PropertyRef,pub imported_as:PropertyRef,pub explicit_as:PropertyRef,pub is_wildcard:PropertyRef,pub is_explicit:PropertyRef,pub is_module_import:PropertyRef,pub imports:EdgeRef,pub is_call_for_import:EdgeRef,pub secondary_id:PropertyRef,pub generic_signature:PropertyRef,}impl HiddenSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_method_schema:&MethodSchema,_arg_type_decl_schema:&TypeSchema,_arg_ast_schema:&AstSchema,_arg_cfg_schema:&CfgSchema,_arg_fs_schema:&FileSystemSchema,_arg_call_graph:&CallGraphSchema,_arg_pdg:&PdgSchema,_arg_tags:&TagsSchema,_arg_shortcuts:&ShortcutsSchema,)->Result<Self>{let _object_1=_arg_base.version.clone();let _object_129=_arg_type_decl_schema.type_decl.clone();let _object_132=_arg_type_decl_schema.member.clone();let _object_138=_arg_method_schema.method.clone();let _object_14=_arg_base.r#ref.clone();let _object_140=_arg_method_schema.method_parameter_in.clone();let _object_142=_arg_method_schema.method_return.clone();let _object_144=_arg_ast_schema.ast_node.clone();let _object_145=_arg_ast_schema.block.clone();let _object_146=_arg_ast_schema.literal.clone();let _object_147=_arg_ast_schema.local.clone();let _object_148=_arg_ast_schema.identifier.clone();let _object_168=_arg_ast_schema.method_ref.clone();let _object_169=_arg_ast_schema.type_ref.clone();let _object_188=_arg_ast_schema.unknown.clone();let _object_189=_arg_ast_schema.ast.clone();let _object_202=_arg_ast_schema.call_node.clone();let _object_203=_arg_ast_schema.expression.clone();let _object_208=_arg_call_graph.evaluation_strategy.clone();let _object_21=_arg_fs_schema.file.clone();let _object_214=_arg_call_graph.argument.clone();let _object_226=_arg_pdg.reaching_def.clone();let _object_228=_arg_shortcuts.contains.clone();let _object_231=_arg_tags.tagged_by.clone();let _object_232=_arg_tags.tag.clone();let _object_6=_arg_base.name.clone();let info_19=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Hidden$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Hidden".to_owned(),schema_name:"Hidden".to_owned(), doc_index:-1,description:"\nThe schema elements defined here are NOT included on the schema website.\nThese are schema elements that we are using but would like to rework\nin the future and not make part of the standard.\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:1401
let _object_246=builder.add_property("CLOSURE_BINDING_ID".to_owned(),ValueType::String,"Identifier which uniquely describes a CLOSURE_BINDING. This property is used to match captured LOCAL nodes with the corresponding CLOSURE_BINDING nodes",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1402
_object_246.proto_id(50);
// CPG_CONSTRUCTION_OP:1403
let _object_247=builder.add_edge_type("CAPTURE".to_owned(),"Represents the capturing of a variable into a closure",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1404
_object_247.proto_id(40);
// CPG_CONSTRUCTION_OP:1405
_object_147.add_property(&_object_246);
// CPG_CONSTRUCTION_OP:1406
_object_140.add_property(&_object_246);
// CPG_CONSTRUCTION_OP:1407
let _object_248=builder.add_node_type("CLOSURE_BINDING".to_owned(),"Represents the binding of a LOCAL or METHOD_PARAMETER_IN into the closure of a method",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1408
_object_248.proto_id(334);
// CPG_CONSTRUCTION_OP:1409
_object_248.add_property(&_object_246);
// CPG_CONSTRUCTION_OP:1410
_object_248.add_property(&_object_208);
// CPG_CONSTRUCTION_OP:1411
let _object_249=builder.add_edge_type("CAPTURED_BY".to_owned(),"Connection between a captured LOCAL and the corresponding CLOSURE_BINDING",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1412
_object_249.proto_id(41);
// CPG_CONSTRUCTION_OP:1413
_object_147.add_out_edge(&_object_249,&_object_248,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1414
_object_140.add_out_edge(&_object_249,&_object_248,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1415
_object_168.add_out_edge(&_object_247,&_object_248,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1416
_object_169.add_out_edge(&_object_247,&_object_248,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1417
_object_248.add_out_edge(&_object_14,&_object_147,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1418
_object_248.add_out_edge(&_object_14,&_object_140,EdgeConnection{out_cardinality:EdgeCardinality::ZeroOrOne,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1419
let _object_250=builder.add_node_type("TEMPLATE_DOM".to_owned(),"This node represents a DOM node used in template languages, e.g., JSX/TSX",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1420
_object_250.proto_id(417);
// CPG_CONSTRUCTION_OP:1421
_object_250.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:1422
_object_250.extend(&_object_203)?;
// CPG_CONSTRUCTION_OP:1423
_object_250.add_out_edge(&_object_189,&_object_203,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1424
_object_250.add_out_edge(&_object_226,&_object_203,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1425
_object_250.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1426
_object_203.add_out_edge(&_object_214,&_object_250,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1427
_object_138.add_out_edge(&_object_228,&_object_250,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1428
_object_21.add_out_edge(&_object_228,&_object_250,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1429
let _object_251=builder.add_property("DEPENDENCY_GROUP_ID".to_owned(),ValueType::String,"The group ID for a dependency",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1430
_object_251.proto_id(58);
// CPG_CONSTRUCTION_OP:1431
let _object_252=builder.add_node_type("DEPENDENCY".to_owned(),"This node represents a dependency",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1432
_object_252.proto_id(35);
// CPG_CONSTRUCTION_OP:1433
_object_252.add_property(&_object_1);
// CPG_CONSTRUCTION_OP:1434
_object_252.add_property(&_object_6);
// CPG_CONSTRUCTION_OP:1435
_object_252.add_property(&_object_251);
// CPG_CONSTRUCTION_OP:1436
_object_252.primary_key(Some(&_object_6));
// CPG_CONSTRUCTION_OP:1437
let _object_253=builder.add_property("DYNAMIC_TYPE_HINT_FULL_NAME".to_owned(),ValueType::String,"Type hint for the dynamic type. These are observed to be verifiable at runtime.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1438
_object_253.as_list();
// CPG_CONSTRUCTION_OP:1439
_object_253.proto_id(1591);
// CPG_CONSTRUCTION_OP:1440
let _object_254=builder.add_property("POSSIBLE_TYPES".to_owned(),ValueType::String,"Similar to `DYNAMIC_TYPE_HINT_FULL_NAME`, but that this makes no guarantee that types within this property are correct. This property is used to capture observations between node interactions during a 'may-analysis'.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1441
_object_254.as_list();
// CPG_CONSTRUCTION_OP:1442
_object_254.proto_id(1592);
// CPG_CONSTRUCTION_OP:1443
_object_202.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1444
_object_202.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1445
_object_140.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1446
_object_140.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1447
_object_142.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1448
_object_142.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1449
_object_168.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1450
_object_168.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1451
_object_169.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1452
_object_169.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1453
_object_147.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1454
_object_147.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1455
_object_145.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1456
_object_145.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1457
_object_188.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1458
_object_188.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1459
_object_146.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1460
_object_146.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1461
_object_148.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1462
_object_148.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1463
_object_132.add_property(&_object_253);
// CPG_CONSTRUCTION_OP:1464
_object_132.add_property(&_object_254);
// CPG_CONSTRUCTION_OP:1465
let _object_255=builder.add_node_type("IMPORT".to_owned(),"Declarative import as it is found in statically typed languages like Java.\nThis kind of node is not supposed to be used for imports in dynamically typed\nlanguages like Javascript.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1466
_object_255.extend(&_object_144)?;
// CPG_CONSTRUCTION_OP:1467
_object_255.starter_name(Some("imports".to_owned()));
// CPG_CONSTRUCTION_OP:1468
let _object_256=builder.add_property("IMPORTED_ENTITY".to_owned(),ValueType::String,"The identifying string of the imported entity.\nFor a Java import like \"import java.nio.ByteBuffer;\" this would be \"java.nio.ByteBuffer\".",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1469
let _object_257=builder.add_property("IMPORTED_AS".to_owned(),ValueType::String,"The identifier under which the import can be accessed in the importing context.\nFor a Java import this is always identical to the class name. But e.g. for a\nKotlin import like \"import java.nio.ByteBuffer as BBuffer\" this would be \"BBuffer\".\nThis property is ignored if IS_WILDCARD is true.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1470
let _object_258=builder.add_property("EXPLICIT_AS".to_owned(),ValueType::Boolean,"Specifies whether the IMPORTED_AS property was explicitly present in the code.\nFor languages like Java which do not allow a renaming during import this is\nalways false. For e.g. Kotlin it depends on the existence of the \"as\" keyword.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1471
let _object_259=builder.add_property("IS_WILDCARD".to_owned(),ValueType::Boolean,"Specifies whether this is a wildcard import.\nFor a Java import like \"import java.nio.*;\" IS_WILDCARD would be \"true\" and\nIMPORTED_ENTITY would be \"java.nio\".\nFor wildcard imports the IMPORTED_AS property is ignored.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1472
let _object_260=builder.add_property("IS_EXPLICIT".to_owned(),ValueType::Boolean,"Specifies whether this is an explicit import.\nMost languages have implicit default imports of some standard library elements\nand this flag is used to distinguish those from explicit imports found in the\ncode base.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1473
let _object_261=builder.add_property("IS_MODULE_IMPORT".to_owned(),ValueType::Boolean,"Specifies whether this is a module import.\nThis is used for languages like Java >= 25 where packages exported by a module\ncan be imported via the module name (which does not need to match the package names in\nany way).",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1474
_object_255.add_property(&_object_256);
// CPG_CONSTRUCTION_OP:1475
_object_255.add_property(&_object_257);
// CPG_CONSTRUCTION_OP:1476
_object_255.add_property(&_object_258);
// CPG_CONSTRUCTION_OP:1477
_object_255.add_property(&_object_259);
// CPG_CONSTRUCTION_OP:1478
_object_255.add_property(&_object_260);
// CPG_CONSTRUCTION_OP:1479
_object_255.add_property(&_object_261);
// CPG_CONSTRUCTION_OP:1480
let _object_262=builder.add_edge_type("IMPORTS".to_owned(),"Edge from imports to dependencies",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1481
_object_262.proto_id(23663);
// CPG_CONSTRUCTION_OP:1482
let _object_263=builder.add_edge_type("IS_CALL_FOR_IMPORT".to_owned(),"Edge from CALL statement in the AST to the IMPORT.\n￼        |We use this edge to traverse from the logical representation of the IMPORT\n￼        |to the corresponding import statement in the AST.\n￼        |",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1483
_object_263.proto_id(23664);
// CPG_CONSTRUCTION_OP:1484
_object_255.add_out_edge(&_object_262,&_object_252,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1485
_object_255.add_out_edge(&_object_231,&_object_232,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1486
_object_202.add_out_edge(&_object_263,&_object_255,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1487
_object_145.add_out_edge(&_object_189,&_object_255,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1488
_object_21.add_out_edge(&_object_189,&_object_255,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1489
_object_129.add_out_edge(&_object_189,&_object_255,EdgeConnection{out_cardinality:EdgeCardinality::List,in_cardinality:EdgeCardinality::List, out_step:"".to_owned(),out_doc:"".to_owned(),in_step:"".to_owned(),in_doc:"".to_owned()});
// CPG_CONSTRUCTION_OP:1490
let _object_264=builder.add_property("SECONDARY_ID".to_owned(),ValueType::String,"ID from a different context, e.g. if the graph was imported from a different format,\nwe can use this to preserve the link to the original",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1491
let _object_265=builder.add_property("GENERIC_SIGNATURE".to_owned(),ValueType::String,"This field is experimental. It will likely be removed in the future without any notice.\nIt stores type information for generic types and methods as well as type information\nfor members and locals where the type either contains a type parameter reference or\nan instantiated type reference.",Some(info_19.clone()));
// CPG_CONSTRUCTION_OP:1492
_object_265.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:1493
_object_265.proto_id(3000);
// CPG_CONSTRUCTION_OP:1494
_object_138.add_property(&_object_265);
// CPG_CONSTRUCTION_OP:1495
_object_129.add_property(&_object_265);
// CPG_CONSTRUCTION_OP:1496
_object_132.add_property(&_object_265);
// CPG_CONSTRUCTION_OP:1497
_object_147.add_property(&_object_265);
 Ok(Self{closure_binding_id:_object_246,capture:_object_247,closure_binding:_object_248,captured_by:_object_249,template_dom:_object_250,dependency_group_id:_object_251,dependency:_object_252,dynamic_type_hint_full_name:_object_253,possible_types:_object_254,import_node:_object_255,imported_entity:_object_256,imported_as:_object_257,explicit_as:_object_258,is_wildcard:_object_259,is_explicit:_object_260,is_module_import:_object_261,imports:_object_262,is_call_for_import:_object_263,secondary_id:_object_264,generic_signature:_object_265,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_base:&BaseSchema,_arg_method_schema:&MethodSchema,_arg_type_decl_schema:&TypeSchema,_arg_ast_schema:&AstSchema,_arg_cfg_schema:&CfgSchema,_arg_fs_schema:&FileSystemSchema,_arg_call_graph:&CallGraphSchema,_arg_pdg:&PdgSchema,_arg_tags:&TagsSchema,_arg_shortcuts:&ShortcutsSchema,)->Result<Self>{Self::new(builder,_arg_base,_arg_method_schema,_arg_type_decl_schema,_arg_ast_schema,_arg_cfg_schema,_arg_fs_schema,_arg_call_graph,_arg_pdg,_arg_tags,_arg_shortcuts,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Hidden$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Hidden".to_owned(),schema_name:"Hidden".to_owned(), doc_index:-1,description:"\nThe schema elements defined here are NOT included on the schema website.\nThese are schema elements that we are using but would like to rework\nin the future and not make part of the standard.\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for HiddenSchema { fn doc_index(&self)->i32{-1} fn description(&self)->&str{"\nThe schema elements defined here are NOT included on the schema website.\nThese are schema elements that we are using but would like to rework\nin the future and not make part of the standard.\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:Hidden
// CPG_SCHEMA_MODULE:ProtoSerialize
#[derive(Clone,Debug)] pub struct ProtoSerializeSchema {pub contained_ref:PropertyRef,}impl ProtoSerializeSchema { pub fn new(builder:&mut ObjectSchemaBuilder,_arg_ast:&AstSchema,)->Result<Self>{let _object_188=_arg_ast.unknown.clone();let info_20=SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize".to_owned(),schema_name:"ProtoSerialize".to_owned(), doc_index:2147483647,description:"\n\n".to_owned(),provided_by_frontend:false};// CPG_CONSTRUCTION_OP:1498
let _object_266=builder.add_property("CONTAINED_REF".to_owned(),ValueType::String,"References to other nodes. This is not a real property; it exists here for the sake of proto serialization only. valueType and cardinality are meaningless.",Some(info_20.clone()));
// CPG_CONSTRUCTION_OP:1499
_object_266.mandatory(Scalar::String { value:"<empty>".to_owned() })?;
// CPG_CONSTRUCTION_OP:1500
_object_266.proto_id(2007161);
// CPG_CONSTRUCTION_OP:1501
_object_188.add_property(&_object_266);
 Ok(Self{contained_ref:_object_266,})} pub fn apply(builder:&mut ObjectSchemaBuilder,_arg_ast:&AstSchema,)->Result<Self>{Self::new(builder,_arg_ast,)} pub fn schema_info()->SchemaInfo{SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize".to_owned(),schema_name:"ProtoSerialize".to_owned(), doc_index:2147483647,description:"\n\n".to_owned(),provided_by_frontend:false}} } impl SchemaBase for ProtoSerializeSchema { fn doc_index(&self)->i32{2147483647} fn description(&self)->&str{"\n\n"} fn provided_by_frontend(&self)->bool{false} }// CPG_SCHEMA_MODULE_END:ProtoSerialize
// CPG_SCHEMA_COMPOSITION:CpgSchema
#[derive(Clone,Debug)] pub struct CpgSchema{pub base:BaseSchema,pub file_system:FileSystemSchema,pub namespace:NamespaceSchema,pub operators:OperatorsSchema,pub configuration:ConfigurationSchema,pub meta_data:MetaDataSchema,pub r#type:TypeSchema,pub method:MethodSchema,pub ast:AstSchema,pub call_graph:CallGraphSchema,pub cfg:CfgSchema,pub dominators:DominatorsSchema,pub pdg:PdgSchema,pub shortcuts:ShortcutsSchema,pub comment:CommentSchema,pub tags:TagsSchema,pub binding:BindingSchema,pub annotation:AnnotationSchema,pub finding:FindingSchema,pub hidden:HiddenSchema,pub proto_serialize:ProtoSerializeSchema,} impl CpgSchema {pub fn new(builder:&mut ObjectSchemaBuilder)->Result<Self>{let base=BaseSchema::new(builder,)?;let file_system=FileSystemSchema::new(builder,&base,)?;let namespace=NamespaceSchema::new(builder,&base,&file_system,)?;let operators=OperatorsSchema::new(builder,)?;let configuration=ConfigurationSchema::new(builder,&base,)?;let meta_data=MetaDataSchema::new(builder,&base,)?;let r#type=TypeSchema::new(builder,&base,&file_system,)?;let method=MethodSchema::new(builder,&base,&r#type,&file_system,)?;let ast=AstSchema::new(builder,&base,&namespace,&method,&r#type,&file_system,)?;let call_graph=CallGraphSchema::new(builder,&method,&ast,)?;let cfg=CfgSchema::new(builder,&method,&ast,)?;let dominators=DominatorsSchema::new(builder,&method,&ast,)?;let pdg=PdgSchema::new(builder,&method,&ast,)?;let shortcuts=ShortcutsSchema::new(builder,&base,&method,&ast,&r#type,&file_system,)?;let comment=CommentSchema::new(builder,&base,&ast,&file_system,)?;let tags=TagsSchema::new(builder,&base,&r#type,&method,&ast,&file_system,&call_graph,)?;let binding=BindingSchema::new(builder,&base,&r#type,&method,&call_graph,)?;let annotation=AnnotationSchema::new(builder,&base,&method,&r#type,&ast,&shortcuts,)?;let finding=FindingSchema::new(builder,&base,)?;let hidden=HiddenSchema::new(builder,&base,&method,&r#type,&ast,&cfg,&file_system,&call_graph,&pdg,&tags,&shortcuts,)?;let proto_serialize=ProtoSerializeSchema::new(builder,&ast,)?; Ok(Self{base,file_system,namespace,operators,configuration,meta_data,r#type,method,ast,call_graph,cfg,dominators,pdg,shortcuts,comment,tags,binding,annotation,finding,hidden,proto_serialize,})}} pub fn populate(builder:&mut ObjectSchemaBuilder)->Result<()>{CpgSchema::new(builder)?;Ok(())}// CPG_SCHEMA_COMPOSITION_END:CpgSchema
// CPG_CONSTRUCTION_END
pub fn schema_parts()->Vec<SchemaInfo>{vec![SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Annotation$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Annotation".to_owned(),schema_name:"Annotation".to_owned(), doc_index:20,description:"\nJava Annotation related CPG definitions.\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Ast$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Ast".to_owned(),schema_name:"Ast".to_owned(), doc_index:7,description:"\nThe Abstract Syntax Tree (AST) Layer provides syntax trees for all compilation units.\nAll nodes of the tree inherit from the same base class (`AST_NODE`) and are connected\nto their child nodes via outgoing `AST` edges.\n\nSyntax trees are typed, that is, when possible, types for all expressions are stored\nin the tree. Moreover, common control structure types are defined in the specification,\nmaking it possible to translate trees into corresponding control flow graphs if only\nthese common control structure types are used, possibly by desugaring on the side of\nthe language frontend. For cases where this is not an option,\nthe AST specification provides means of storing language-dependent information in the\nAST that can be interpreted by language-dependent control flow construction passes.\n\nThis layer MUST be created by the frontend.\n\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Base$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Base".to_owned(),schema_name:"Base".to_owned(), doc_index:2147483647,description:"\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Binding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Binding".to_owned(),schema_name:"Binding".to_owned(), doc_index:19,description:"\nWe use the concept of \"bindings\" to support resolving of\n(method-name, signature) pairs at type declarations (`TYPE_DECL`). For each\npair that we can resolve, we create a `BINDING` node that is connected the\nthe type declaration via an incoming `BINDS` edge. The `BINDING` node is\nconnected to the method it resolves to via an outgoing `REF` edge.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.CallGraph$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.CallGraph".to_owned(),schema_name:"CallGraph".to_owned(), doc_index:8,description:"\nThe Call Graph Layer represents call relations between methods.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Cfg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Cfg".to_owned(),schema_name:"Cfg".to_owned(), doc_index:9,description:"\nThe Control Flow Graph Layer provides control flow graphs for each method. Control\nflow graphs are constructed by marking a sub set of the abstract syntax tree nodes\nas control flow nodes (`CFG_NODE`) and connecting these nodes via `CFG` edges.\nThe control flow graph models both the control flow within the calculation of an\nexpression as well as from expression to expression. The layer can be automatically\ngenerated from the syntax tree layer if only control structure types supported by\nthis specification are employed.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Comment$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Comment".to_owned(),schema_name:"Comment".to_owned(), doc_index:12,description:"".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Configuration$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Configuration".to_owned(),schema_name:"Configuration".to_owned(), doc_index:18,description:"\nThe code property graph specification currently does not contain\nschema elements for the representation of configuration files in\na structured format, however, it does allow configuration files\nto be included verbatim in the graph to enable language-/framework-\nspecific passes to access them. This layer provides the necessary\nschema elements for this basic support of configuration files.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Dominators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Dominators".to_owned(),schema_name:"Dominators".to_owned(), doc_index:10,description:"\nThe Dominators Layer provides dominator- and post-dominator trees for all methods.\nIt is constructed automatically from the control flow graph layer and is in turn\nused to automatically construct control dependence relations of the PDG layer.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.FileSystem$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.FileSystem".to_owned(),schema_name:"FileSystem".to_owned(), doc_index:3,description:"\nCPGs are created from sets of files and the File System Layer describes the layout of\nthese files, that is, it provides information about source files\nand shared objects for source-based and machine-code-based frontends\nrespectively. The purpose of including this information in the CPG\nis to allow nodes of the graph to be mapped back to file system locations.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Finding$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Finding".to_owned(),schema_name:"Finding".to_owned(), doc_index:15,description:"\nWe allow findings (e.g., potential vulnerabilities, notes on dangerous practices) to\nbe stored in the Findings Layer.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Hidden$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Hidden".to_owned(),schema_name:"Hidden".to_owned(), doc_index:-1,description:"\nThe schema elements defined here are NOT included on the schema website.\nThese are schema elements that we are using but would like to rework\nin the future and not make part of the standard.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.MetaData$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.MetaData".to_owned(),schema_name:"MetaData".to_owned(), doc_index:2,description:"\nThe Meta Data Layer contains information about CPG creation. In particular,\nit indicates which language frontend generated the CPG and which overlays\nhave been applied. The layer consists of a single node - the Meta Data node -\nand language frontends MUST create this node. Overlay creators MUST edit\nthis node to indicate that a layer has been successfully applied in all\ncases where applying the layer more than once is prohibitive.\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Method$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Method".to_owned(),schema_name:"Method".to_owned(), doc_index:5,description:"\nThe Method Layer contains declarations of methods, functions, and procedures.\nInput parameters and output parameters (including return parameters) are\nrepresented, however, method contents is not present in this layer.\n".to_owned(),provided_by_frontend:true},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Namespace$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Namespace".to_owned(),schema_name:"Namespace".to_owned(), doc_index:4,description:"\nMany programming languages allow code to be structured into namespaces. The Namespace\nLayer makes these namespaces explicit and associates program constructs with the\nnamespaces they are defined in.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Operators$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Operators".to_owned(),schema_name:"Operators".to_owned(), doc_index:17,description:"\n\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Pdg$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Pdg".to_owned(),schema_name:"Pdg".to_owned(), doc_index:11,description:"\n      The Program Dependence Graph Layer contains a program dependence graph for\neach method of the source program. A program dependence graph consists\nof a data dependence graph (DDG) and a control dependence graph (CDG),\ncreated by connecting nodes of the control flow graph via `REACHING_DEF`\nand `CDG` edges respectively.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.ProtoSerialize".to_owned(),schema_name:"ProtoSerialize".to_owned(), doc_index:2147483647,description:"\n\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Shortcuts$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Shortcuts".to_owned(),schema_name:"Shortcuts".to_owned(), doc_index:16,description:"\nThe Shortcuts Layer provides shortcut edges calculated to speed up\nsubsequent queries. Language frontends MUST NOT create shortcut edges.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Tags$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Tags".to_owned(),schema_name:"Tags".to_owned(), doc_index:17,description:"\nThe Code Property Graph specification allows for tags to be attached\nto arbitrary nodes. Conceptually, this is similar to the creation of\nFinding nodes, however, tags are to be used for intermediate results\nrather than end-results that are to be reported to the user.\n".to_owned(),provided_by_frontend:false},SchemaInfo {definition_class:"io.shiftleft.codepropertygraph.schema.Type$Schema".to_owned(),schema_class:"io.shiftleft.codepropertygraph.schema.Type".to_owned(),schema_name:"Type".to_owned(), doc_index:6,description:"\nThe Type Layer contains information about type declarations, relations between\ntypes, and type instantiation and usage. In its current form, it allows modelling of\nparametrized types, type hierarchies and aliases.\n".to_owned(),provided_by_frontend:true},]}// CPG_SCHEMA_FACTORY:domain
pub fn domain_schema_objects()->Result<ObjectSchema>{let mut builder=ObjectSchemaBuilder::new("Cpg".to_owned(),"io.shiftleft.codepropertygraph.generated".to_owned(),vec!["io.shiftleft".to_owned(),]); populate(&mut builder)?;builder.build()} pub fn domain_schema()->Result<Schema>{domain_schema_objects()?.to_model()}// CPG_SCHEMA_FACTORY:protobuf
pub fn protobuf_schema_objects()->Result<ObjectSchema>{let mut builder=ObjectSchemaBuilder::new("Cpg".to_owned(),"io.shiftleft.codepropertygraph.generated".to_owned(),vec![]);builder.proto_options(Some(ProtoOptions {package:"cpg".to_owned(),java_outer_classname:"Cpg".to_owned(),java_package:"io.shiftleft.proto.cpg".to_owned(),go_package:"github.com/ShiftLeftSecurity/proto/cpg".to_owned(),csharp_namespace:"io.shiftleft.proto.cpg".to_owned(), uncommon_proto_enum_name_mappings:std::collections::BTreeMap::from([("ControlStructureTypes".to_owned(),"CONTROL_STRUCTURE_TYPES".to_owned()),("Frameworks".to_owned(),"FRAMEWORKS".to_owned()),("Languages".to_owned(),"LANGUAGES".to_owned()),])})); populate(&mut builder)?;builder.build()} pub fn protobuf_schema()->Result<Schema>{protobuf_schema_objects()?.to_model()}// CPG_PROTO_IDS
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