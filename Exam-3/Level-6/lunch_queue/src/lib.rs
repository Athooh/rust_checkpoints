pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }

    pub fn add(&mut self, name: String, discount: i32) {
        let new_person = Box::new(Person {
            name,
            discount,
            next_person: None,
        });

        match self.node {
            Some(ref mut last) => {
                let mut current = last;
                while let Some(ref mut next) = current.next_person {
                    current = next;
                }
                current.next_person = Some(new_person);
            }
            None => self.node = Some(new_person),
        }
    }

    pub fn invert_queue(&mut self) {
        let mut prev = None;
        let mut current = self.node.take();

        while let Some(mut node) = current {
            let next = node.next_person.take();
            node.next_person = prev;
            prev = Some(node);
            current = next;
        }

        self.node = prev;
    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        self.node.take().map(|node| {
            self.node = node.next_person;
            (node.name, node.discount)
        })
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = &self.node;

        while let Some(ref node) = current {
            if node.name == name {
                return Some((node.name.clone(), node.discount));
            }
            current = &node.next_person;
        }

        None
    }
}
