
pub struct Node {
    name: String,
    children: Vec<Node>
}


impl Node {
    pub fn new(name: String) -> Self {
        Node {name, children: Vec::<Node>::new()}
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

}
