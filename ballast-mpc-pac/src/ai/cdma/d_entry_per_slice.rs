#[doc = "Register `D_ENTRY_PER_SLICE` reader"]
pub struct R(crate::R<D_ENTRY_PER_SLICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_ENTRY_PER_SLICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_ENTRY_PER_SLICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_ENTRY_PER_SLICE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENTRIES` reader - "]
pub struct ENTRIES_R(crate::FieldReader<u16>);
impl ENTRIES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ENTRIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENTRIES_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn entries(&self) -> ENTRIES_R {
        ENTRIES_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Number of CBUF entries used for one input slice\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_entry_per_slice](index.html) module"]
pub struct D_ENTRY_PER_SLICE_SPEC;
impl crate::RegisterSpec for D_ENTRY_PER_SLICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_entry_per_slice::R](R) reader structure"]
impl crate::Readable for D_ENTRY_PER_SLICE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_ENTRY_PER_SLICE to value 0"]
impl crate::Resettable for D_ENTRY_PER_SLICE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
