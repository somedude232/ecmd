extern crate core;

use layer2::nf_frame::nf_frame;

pub struct nf_cache {
    frames: Vec<nf_frame>,
}

impl nf_cache {
    pub fn new() -> nf_cache {
        nf_cache { frames: Vec::new() }
    }

    pub fn from_vec(vec: Vec<nf_frame>) -> nf_cache {
        nf_cache { frames: vec }
    }

    pub fn push(&mut self, frame: nf_frame) {
        self.frames.push(frame);
    }

    pub fn get(&self, index: usize) -> Option<&nf_frame> {
        (*self.frames).get(index)
    }

    pub fn get_frames(&mut self) -> &Vec<nf_frame> {
        &self.frames
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }
}

trait SliceExt<T> {
    type Item;

    fn get<I>(&self, index: I) -> &I::Output
        where I: SliceIndex<Self::Item>;
}

impl<T> SliceExt<T> for [T] {
    type Item = T;

    fn get<I>(&self, index: I) -> &I::Output
        where I: SliceIndex<T>
    {
        panic!()
    }
}

pub trait SliceIndex<T> {
    type Output: ?Sized;
}

impl<T> SliceIndex<T> for usize {
    type Output = T;
}
