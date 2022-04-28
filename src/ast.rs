#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    CALL,
    VAR,
    STRING,
    ROOT,
    FN,
    ALIAS,
    NOOP,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    kind: NodeKind,
    name: String,
    nodes: Vec<Node>,
    args: Vec<Node>,
    print: bool,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Node {
            kind,
            name: "".to_string(),
            nodes: Vec::<Node>::new(),
            args: Vec::<Node>::new(),
            print: false
        }
    }

    pub fn get_kind(&self) -> NodeKind {
        self.kind.clone()
    }

    pub fn add_node(&mut self, node: Node) -> &Self {
        self.nodes.push(node);
        self
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn set_name(&mut self, name: String) -> &Self {
        self.name = name;
        self
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_arg(&mut self, node: Node) -> &Self {
        self.args.push(node);
        self
    }

    pub fn get_args(&self) -> &Vec<Node> {
        &self.args
    }
}