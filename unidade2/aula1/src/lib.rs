// Testes Simples para Implementação de Árvore Binária de Busca (BST)
// Os alunos devem implementar a árvore para fazer estes testes passarem

#[cfg(test)]
mod bst_tests {
    use crate::BST;
    // Importe sua implementação de BST aqui
    // use crate::BST;
    
    #[test]
    fn test_bst_new_and_empty() {
        // Teste 1: Criar uma nova árvore e verificar se está vazia
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        // Teste 2: Inserir elementos e verificar se estão na árvore
        let mut bst = BST::new();
        
        // Inserir alguns valores
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        // Verificar se os valores inseridos estão na árvore
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        
        // Verificar que um valor não inserido não está na árvore
        assert!(!bst.search(20));
        
        // A árvore não deve mais estar vazia
        assert!(!bst.is_empty());
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct BST {
    pub root: Option<Box<Node>>,
}

impl BST {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, value: i32) {
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(current: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        match current {
            None => Some(Box::new(Node { value, left: None, right: None })),
            Some(mut node) => {
                if value < node.value {
                    node.left = Self::insert_node(node.left.take(), value);
                } else {
                    node.right = Self::insert_node(node.right.take(), value);
                }
                Some(node)
            }
        }
    }

    pub fn search(&self, value: i32) -> bool {
        Self::search_node(&self.root, value)
    }

    fn search_node(current: &Option<Box<Node>>, value: i32) -> bool {
        match current {
            None => false,
            Some(node) => {
                if node.value == value {
                    return true
                } else if value < node.value {
                    Self::search_node(&node.left, value)
                } else {
                    Self::search_node(&node.right, value)
                }
            }
        }
    }
}