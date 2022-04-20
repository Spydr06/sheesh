pub enum NodeKind {
    CALL,
    PIPE,
    EXECUTE,
    VAR,
    EXIT,
}

pub struct Node {
    kind: NodeKind,

}