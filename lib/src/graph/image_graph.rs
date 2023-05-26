use crate::graph::{ImageEdge, ImageNode, ImageNodeColor};
use std::cell::Cell;

/// Represents an image graph, consisting of one node per pixel which are 4-connected.
#[derive(Debug, Clone, Default)]
pub struct ImageGraph {
    /// Number of components.
    k: Cell<usize>,
    /// All nodes in this graph.
    nodes: Nodes,
    /// All edges in this graph.
    edges: Edges,
}

#[derive(Debug, Clone, Default)]
pub struct Nodes {
    nodes: Vec<Cell<ImageNode>>,
    node_colors: Vec<Cell<ImageNodeColor>>,
}

#[derive(Debug, Clone, Default)]
pub struct Edges {
    edges: Vec<Cell<ImageEdge>>,
}

impl ImageGraph {
    /// Constructs an image graph with the given exact number of nodes.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of nodes to allocate.
    pub fn new_with_nodes(n: usize) -> Self {
        Self {
            k: Cell::new(n),
            nodes: Nodes::allocated(n),
            ..Self::default()
        }
    }

    /// Resets the image graph with the given exact number of nodes.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of nodes to allocate.
    #[allow(dead_code)]
    pub fn reset(&mut self, n: usize) {
        self.k.replace(n);
        self.nodes = Nodes::allocated(n);
        self.edges.clear();
    }

    /// Get the number of nodes.
    ///
    /// # Return
    ///
    /// The number of nodes.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Get the number of edges.
    ///
    /// # Return
    ///
    /// The number of edges.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    /// Get the number of connected components.
    ///
    /// # Return
    ///
    /// The number connected components.
    pub fn num_components(&self) -> usize {
        self.k.get()
    }

    /// Merge two pixels (that is merge two nodes).
    ///
    /// # Arguments
    ///
    /// * `s_n` - The first node.
    /// * `s_m` - The second node.
    /// * `e` - The corresponding edge.
    ///
    /// # Remarks
    ///
    /// Depending on the used "Distance", some lines may be commented out
    /// to speed up the algorithm.
    pub fn merge(&self, s_n: &Cell<ImageNode>, s_m: &Cell<ImageNode>, e: &ImageEdge) {
        let mut lhs = s_n.get();
        let mut rhs = s_m.get();
        debug_assert_ne!(lhs.id, rhs.id);

        rhs.label = lhs.id;

        // Update count.
        lhs.n += rhs.n;

        // Update maximum weight.
        lhs.max_w = lhs.max_w.max(rhs.max_w).max(e.w);

        // Update the nodes.
        s_n.set(lhs);
        s_m.set(rhs);

        // Update component count.
        let new_k = self.k.get() - 1;
        self.k.replace(new_k);
    }

    /// Get a reference to the n-th node.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the node.
    ///
    /// # Return
    ///
    /// The node at index `n`.
    pub fn node_at(&self, n: usize) -> &Cell<ImageNode> {
        self.nodes.at(n)
    }

    /// Get a reference to the n-th node.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the node.
    ///
    /// # Return
    ///
    /// The node at index `n`.
    #[inline(always)]
    pub fn node_color_at(&self, n: usize) -> &Cell<ImageNodeColor> {
        self.nodes.color_at(n)
    }

    /// Get the ID of the n-th node.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the node.
    ///
    /// # Return
    ///
    /// The ID of the node at index `n`.
    #[inline(always)]
    pub fn node_id_at(&self, n: usize) -> usize {
        let id = self.nodes.at(n).get().id;
        debug_assert_eq!(id, n); // TODO: Remove this method call.
        id
    }

    /// Gets a reference to the n-th edge.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the edge.
    ///
    /// # Return
    ///
    /// The edge at index `n`.
    pub fn edge_at(&self, n: usize) -> &Cell<ImageEdge> {
        self.edges.at(n)
    }

    /// When two nodes get merged, the first node is assigned the id of the second
    /// node as label. By traversing this labeling, the current component of each
    /// node (that is, pixel) can easily be identified and the label can be updated
    /// for efficiency.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node to find the component for.
    ///
    /// # Returns
    ///
    /// The node representing the found component.
    pub fn find_node_component_at(&self, index: usize) -> usize {
        self.nodes.find_component_at(index)
    }

    /// Add new edges.
    ///
    /// # Arguments
    ///
    /// * `edges` - The edges to add.
    pub fn add_edges<I>(&mut self, edges: I)
    where
        I: Iterator<Item = ImageEdge>,
    {
        self.edges.add_many(edges)
    }

    /// Removes all edges.
    pub fn clear_edges(&mut self) {
        self.edges.clear();
    }

    /// Sorts the edges by weight.
    pub fn sort_edges(&mut self) {
        self.edges.sort_by_weight()
    }
}

impl Nodes {
    pub fn allocated(n: usize) -> Self {
        let nodes = vec![Default::default(); n];
        let colors = vec![Default::default(); n];
        Self {
            nodes,
            node_colors: colors,
        }
    }

    /// Set the node of the given index.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the node.
    /// * `node` - The node to set.
    #[allow(dead_code)]
    pub fn set(&mut self, n: usize, node: ImageNode) {
        assert!(n < self.nodes.len());
        self.nodes[n].replace(node);
    }

    /// Add a new node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to add.
    #[allow(dead_code)]
    pub fn add(&mut self, node: ImageNode) {
        self.nodes.push(Cell::new(node))
    }

    /// Get a reference to the n-th node.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the node.
    ///
    /// # Return
    ///
    /// The node at index `n`.
    pub fn at(&self, n: usize) -> &Cell<ImageNode> {
        assert!(n < self.nodes.len());
        &self.nodes[n]
    }

    /// Get a reference to the n-th node color.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the node color.
    ///
    /// # Return
    ///
    /// The node at index `n`.
    #[inline(always)]
    pub fn color_at(&self, n: usize) -> &Cell<ImageNodeColor> {
        assert!(n < self.node_colors.len());
        &self.node_colors[n]
    }

    /// When two nodes get merged, the first node is assigned the id of the second
    /// node as label. By traversing this labeling, the current component of each
    /// node (that is, pixel) can easily be identified and the label can be updated
    /// for efficiency.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the node to find the component for.
    ///
    /// # Returns
    ///
    /// The node representing the found component.
    pub fn find_component_at(&self, index: usize) -> usize {
        let mut n = self.nodes[index].get();
        debug_assert_eq!(n.id, index);
        if n.label == n.id {
            return index;
        }

        // Get component of node n.
        let mut l = n.label;
        let mut id = n.id;

        while l != id {
            let token = self.nodes[l].get();
            l = token.label;
            id = token.id;
        }

        // If the found component is identical to the originally provided index, we must not borrow again.
        debug_assert_ne!(l, index);

        let s = self.nodes[l].get();
        debug_assert_eq!(s.label, s.id);

        // Save latest component.
        n.label = s.id;
        self.nodes[index].set(n);
        l
    }

    /// Returns the number of nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl Edges {
    /// Add a new edge.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge to add.
    pub fn add(&mut self, edge: ImageEdge) {
        self.edges.push(Cell::new(edge))
    }

    /// Add new edges.
    ///
    /// # Arguments
    ///
    /// * `edges` - The edges to add.
    pub fn add_many<I>(&mut self, edges: I)
    where
        I: Iterator<Item = ImageEdge>,
    {
        for edge in edges.into_iter() {
            self.add(edge);
        }
    }

    /// Gets a reference to the n-th edge.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the edge.
    ///
    /// # Return
    ///
    /// The edge at index `n`.
    pub fn at(&self, n: usize) -> &Cell<ImageEdge> {
        assert!(n < self.edges.len());
        &self.edges[n]
    }

    /// Sorts the edges by weight.
    pub fn sort_by_weight(&mut self) {
        self.edges.sort_by(|a, b| {
            let a = a.get();
            let b = b.get();

            // Main sorting is by edge weight ascending.
            // In order to improve cache coherency during processing, we then sort by index.
            let ord_w = a.w.partial_cmp(&b.w).unwrap();
            let ord_n = a.n.partial_cmp(&b.n).unwrap();
            let ord_m = a.m.partial_cmp(&b.m).unwrap();
            ord_w.then(ord_n).then(ord_m)
        });
    }

    /// Removes all edges.
    pub fn clear(&mut self) {
        self.edges.clear()
    }

    /// Returns the number of edges.
    pub fn len(&self) -> usize {
        self.edges.len()
    }
}
