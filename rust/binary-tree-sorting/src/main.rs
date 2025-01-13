#[derive(Debug, Clone)]
struct AVLNode {
    value: i64,
    height: i32,
    left: Option<Box<AVLNode>>,
    right: Option<Box<AVLNode>>,
}

impl AVLNode {
    fn new(value: i64) -> Self {
        AVLNode {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height(node: &Option<Box<AVLNode>>) -> i32 {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    fn update_height(node: &mut Option<Box<AVLNode>>) {
        if let Some(n) = node {
            n.height = 1 + std::cmp::max(Self::height(&n.left), Self::height(&n.right));
        }
    }

    fn balance_factor(node: &Option<Box<AVLNode>>) -> i32 {
        match node {
            Some(n) => Self::height(&n.left) - Self::height(&n.right),
            None => 0,
        }
    }

    fn rotate_right(y: &mut Option<Box<AVLNode>>) -> Option<Box<AVLNode>> {
        let mut x = y.as_mut()?.left.take()?;
        y.as_mut()?.left = x.right.take();
        x.right = y.take();
        Self::update_height(&mut x.right);
        Self::update_height(&mut Some(Box::new(*x.clone())));
        Some(x)
    }

    fn rotate_left(x: &mut Option<Box<AVLNode>>) -> Option<Box<AVLNode>> {
        let mut y = x.as_mut()?.right.take()?;
        x.as_mut()?.right = y.left.take();
        y.left = x.take();
        Self::update_height(&mut y.left);
        Self::update_height(&mut Some(Box::from(y.clone())));
        Some(y)
    }

    fn balance(node: &mut Option<Box<AVLNode>>) {
        Self::update_height(node);
        let balance = Self::balance_factor(node);
        if balance > 1 {
            if let Some(ref mut left) = node.as_mut().and_then(|n| n.left.as_mut()) {
                if Self::balance_factor(&Some(Box::from(left.clone()))) < 0 {
                    node.as_mut().unwrap().left =
                        Self::rotate_left(&mut node.as_mut().unwrap().left);
                }
            }
            *node = Self::rotate_right(node);
        } else if balance < -1 {
            if let Some(ref mut right) = node.as_mut().and_then(|n| n.right.as_mut()) {
                if Self::balance_factor(&Some(Box::from(right.clone()))) > 0 {
                    node.as_mut().unwrap().right =
                        Self::rotate_right(&mut node.as_mut().unwrap().right);
                }
            }
            *node = Self::rotate_left(node);
        }
    }

    fn insert(node: &mut Option<Box<AVLNode>>, value: i64) {
        if let Some(n) = node {
            if value < n.value {
                Self::insert(&mut n.left, value);
            } else {
                Self::insert(&mut n.right, value);
            }
        } else {
            *node = Some(Box::new(AVLNode::new(value)));
        }
        Self::balance(node);
    }

    fn in_order_traversal(node: &Option<Box<AVLNode>>, result: &mut Vec<i64>)
    where
        i64: Clone,
    {
        if let Some(n) = node {
            Self::in_order_traversal(&n.left, result);
            result.push(n.value.clone());
            Self::in_order_traversal(&n.right, result);
        }
    }
}

fn main() {
    let mut root: Option<Box<AVLNode>> = None;
    let values = vec![20, 10, 34, 83, 124, 31];
    for value in values {
        AVLNode::insert(&mut root, value);
    }

    let mut sorted_values = Vec::new();
    AVLNode::in_order_traversal(&root, &mut sorted_values);
    println!("{:?}", sorted_values);
}
