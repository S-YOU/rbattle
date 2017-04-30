use graph::{Graph, Node};
use std::ops::Range;

/// A point in the graph's two-dimensional space.
pub type Point = (f32, f32);

/// A `Graph` that can be drawn on the screen.
///
/// Each node of a `VisibleGraph` has a designated center position.
///
/// Each node occupies a particular area of the screen: imagine squares, hexes,
/// or whatever. Different nodes' areas do not overlap. Naturally, a node's area
/// includes its center position.
///
/// If two nodes are neighbors, then their areas must be in contact along some
/// line segment.
///
/// To make the graph easier to work with in the game, a `VisibleGraph` promises
/// that a straight line drawn from the center of one node to another must not
/// cross over any third node's area. (This restricts the sorts of graphs we can
/// use: Voronoi diagrams can have nodes that are neighbors, but where a
/// straight line from their centers crosses over some third node's territory.
/// But this rule makes drawing goop flow lines easier, and still permits
/// various sorts of graphs.)
///
/// # Coordinate systems
///
/// A `VisibleGraph` uses its own coordinate space, called "graph space". Its
/// nodes' areas always fall in within some axis-aligned bounding box
/// (0,0)..(max_x,max_y). You can obtain `(max_x,max_y)` by calling the graph's
/// `bounds` method.
///
/// # Boundary lines
///
/// To help draw the graph, a `VisibleGraph` can list the line segments that
/// bound any node's area, and name the node whose area lies on the other side
/// of each line segment (if any). For hit detection, the `VisibleGraph` can
/// find the boundary line segment closest to a given point.
///
/// Since the line segments that mark the boundaries of a node's area form a
/// closed loop, each line segment shares each of its endpoints with the next
/// segment. And when there is another node on the other side of the segment
/// (that is, the boundary isn't part of the exterior boundary of the entire
/// graph), it may share its endpoints with even more segments.
///
/// To avoid repeating endpoint coordinates, the `VisibleGraph` provides all
/// coordinate pairs as a vector of points. Then, when listing the line segments
/// that bound a given node's area, the `VisibleGraph` describes each segment
/// not as a pair of points, but as a pair of indices into the vector.
///
/// OpenGL prefers to have actual points and drawable items separated in this
/// way, because sharing vertex positions reduces the amount of data that must
/// be moved from CPU to GPU to draw a given frame.

pub trait VisibleGraph: Graph {
    /// Return the bounds of the entire graph as a pair `(max_x, max_y)`. The
    /// areas of all nodes in the graph fall in the rectangle
    /// `(0..max_x, 0..max-y)`.
    fn bounds(&self) -> (f32, f32);

    /// Return the center of `node`.
    fn center(&self, node: Node) -> Point;

    /// Return the radius of the largest circle centered on `node`
    /// that lies entirely within its area. The game is easier to play
    /// when most nodes have similar radii.
    fn radius(&self, node: Node) -> f32;

    /// Return a vector of the line segments that bound `node`'s area.
    fn boundary(&self, node: Node) -> Vec<IndexedSegment>;

    /// Return a vector holding all boundaries' line segments' endpoint
    /// coordinates. The `boundary` iterator refers to these positions by their
    /// index.
    fn endpoints(&self) -> Vec<Point>;

    /// Determine which boundary line a mouse click on the point `(x, y)` refers
    /// to. If it refers to a boundary line between two nodes, return them.
    /// Otherwise, return `None`.
    fn boundary_hit(&self, &Point) -> Option<(Node, Node)>;
}

/// A line segment from the boundary of a node's area.
#[derive(Clone, Debug)]
pub struct IndexedSegment {
    /// The `start` and `end` fields of this range are the indices of the line
    /// segment's start and end `Points` in the vector returned by
    /// `VisibleGraph::endpoints()`.
    pub line: Range<usize>,

    /// The node on the other side of the line segment, if any.
    /// We can use this to make sure we draw line segments shared by other
    /// nodes' boundaries only once.
    pub neighbor: Option<Node>
}