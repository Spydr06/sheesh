#[derive(Debug, Clone)]
pub enum NodeKind {
    CALL,
    VAR,
    ID,
    STRING,
    ROOT,
    FN,
    ALIAS,
    NOOP,
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: NodeKind,
    name: String,
    nodes: Vec<Node>,
    args: Vec<Node>
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Node {
            kind,
            name: "".to_string(),
            nodes: Vec::<Node>::new(),
            args: Vec::<Node>::new()
        }
    }

    pub fn add_node(&mut self, node: Node) -> &Self {
        self.nodes.push(node);
        self
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
}