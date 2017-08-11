extern crate core;

use layer2::nf_frame::nf_frame;

pub struct nf_cache<'a> {
    frames: &'a mut Vec<nf_frame<'a>>,
}

impl<'a> nf_cache<'a> {
    pub fn new() -> nf_cache<'a> {
        nf_cache { frames: &mut Vec::new() }
    }

    pub fn from_vec(vec: &'a mut Vec<nf_frame<'a>>) -> nf_cache<'a> {
        nf_cache { frames: vec }
    }

    pub fn push(&mut self, frame: nf_frame<'a>) {
        self.frames.push(frame);
    }

    pub fn get(&self, index: usize) -> Option<&nf_frame> {
        (*self.frames).get(index)
    }
    pub fn get_mut_frames(&'a mut self) -> &'a mut Vec<nf_frame<'a>> {
        self.frames
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
