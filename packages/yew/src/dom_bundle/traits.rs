use super::BNode;
use crate::html::AnyScope;
use crate::html::NodeRef;
use web_sys::Element;

/// A Reconcile Target.
///
/// When a [Reconcilable] is attached, a reconcile target is created to store additional
/// information.
pub(super) trait ReconcileTarget {
    /// Remove self from parent.
    ///
    /// Parent to detach is `true` if the parent element will also be detached.
    fn detach(self, parent: &Element, parent_to_detach: bool);

    /// Move elements from one parent to another parent.
    /// This is for example used by `VSuspense` to preserve component state without detaching
    /// (which destroys component state).
    fn shift(&self, next_parent: &Element, next_sibling: NodeRef);
}

/// This trait provides features to update a tree by calculating a difference against another tree.
pub(super) trait Reconcilable {
    type Bundle: ReconcileTarget;

    /// Attach a virtual node to the DOM tree.
    ///
    /// Parameters:
    /// - `parent_scope`: the parent `Scope` used for passing messages to the
    ///   parent `Component`.
    /// - `parent`: the parent node in the DOM.
    /// - `next_sibling`: to find where to put the node.
    ///
    /// Returns a reference to the newly inserted element.
    fn attach(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle);

    /// Scoped diff apply to other tree.
    ///
    /// Virtual rendering for the node. It uses parent node and existing
    /// children (virtual and DOM) to check the difference and apply patches to
    /// the actual DOM representation.
    ///
    /// Parameters:
    /// - `parent_scope`: the parent `Scope` used for passing messages to the
    ///   parent `Component`.
    /// - `parent`: the parent node in the DOM.
    /// - `next_sibling`: the next sibling, used to efficiently find where to
    ///   put the node.
    /// - `bundle`: the node that this node will be replacing in the DOM. This
    ///   method will remove the `bundle` from the `parent` if it is of the wrong
    ///   kind, and otherwise reuse it.
    ///
    /// Returns a reference to the newly inserted element.
    fn reconcile_node(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef;

    fn reconcile(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut Self::Bundle,
    ) -> NodeRef;

    /// Replace an existing bundle by attaching self and detaching the existing one
    fn replace(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef
    where
        Self: Sized,
        Self::Bundle: Into<BNode>,
    {
        let (self_ref, self_) = self.attach(parent_scope, parent, next_sibling);
        let ancestor = std::mem::replace(bundle, self_.into());
        ancestor.detach(parent, false);
        self_ref
    }
}
