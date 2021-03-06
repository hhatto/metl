use cocoa::base::id;
use std::mem;
use std::ops::Deref;
use sys::{MTLMultisampleDepthResolveFilter, MTLRenderPassDepthAttachmentDescriptor};
use {FromRaw, RenderPassAttachmentDescriptor};

pub struct RenderPassDepthAttachmentDescriptor(id);

impl RenderPassDepthAttachmentDescriptor {
    pub fn clear_depth(&self) -> f64 {
        unsafe { self.0.clearDepth() }
    }

    pub fn set_clear_depth(&mut self, clear_depth: f64) {
        unsafe { self.0.setClearDepth(clear_depth) }
    }

    pub fn depth_resolve_filter(&self) -> MultisampleDepthResolveFilter {
        unsafe { self.0.depthResolveFilter().into() }
    }

    pub fn set_depth_resolve_filter(&mut self, resolve_filter: MultisampleDepthResolveFilter) {
        unsafe { self.0.setDepthResolveFilter(resolve_filter.into()) }
    }
}

impl Clone for RenderPassDepthAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe { FromRaw::from_raw(self.0.copy()).unwrap() }
    }
}

impl Deref for RenderPassDepthAttachmentDescriptor {
    type Target = RenderPassAttachmentDescriptor;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}

impl_from_into_raw!(RenderPassDepthAttachmentDescriptor,
                    of class "MTLRenderPathDepthAttachmentDescriptor");

convertible_enum! {
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub enum MultisampleDepthResolveFilter: MTLMultisampleDepthResolveFilter {
        Sample0 => MTLMultisampleDepthResolveFilterSample0,
        Min => MTLMultisampleDepthResolveFilterMin,
        Max => MTLMultisampleDepthResolveFilterMax
    }

}
