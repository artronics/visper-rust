use winit::window::CursorIcon::NoDrop;

pub mod proxy;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node {
    val: isize,
    pub first_child: Option<Box<Node>>,
    pub next_sibling: Option<Box<Node>>,
}

/*
let container = Container::new();
container
    .append(Row::new())
    .append(Col::new())

let container = Row::new()
    .append(Col::new())

let container = Container::Row {
    style: Style::new()

}.append(Container::Col {

}).append(Container::Col{})
let w = Window::new(container);

*/

#[cfg(test)]
mod tests {
    use super::*;
    use indextree::Arena;

    #[test]
    fn foo() {

// Create a new arena
        let arena = &mut Arena::new();

// Add some new nodes to the arena
        let a = arena.new_node("kir");
        let b = arena.new_node("kso");
        let n = &mut Arena::new();
        n.new_node("sd");
        b.append(a, n);

// Append a to b
        a.append(b, arena);
        assert_eq!(b.ancestors(n).into_iter().count(), 2);
    }
}
