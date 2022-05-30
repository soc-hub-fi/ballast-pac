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
#[doc = "Register `D_ENTRY_PER_SLICE` writer"]
pub struct W(crate::W<D_ENTRY_PER_SLICE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_ENTRY_PER_SLICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<D_ENTRY_PER_SLICE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_ENTRY_PER_SLICE_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `ENTRIES` writer - "]
pub struct ENTRIES_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTRIES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn entries(&self) -> ENTRIES_R {
        ENTRIES_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn entries(&mut self) -> ENTRIES_W {
        ENTRIES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of CBUF entries used for one input slice\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_entry_per_slice](index.html) module"]
pub struct D_ENTRY_PER_SLICE_SPEC;
impl crate::RegisterSpec for D_ENTRY_PER_SLICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_entry_per_slice::R](R) reader structure"]
impl crate::Readable for D_ENTRY_PER_SLICE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_entry_per_slice::W](W) writer structure"]
impl crate::Writable for D_ENTRY_PER_SLICE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_ENTRY_PER_SLICE to value 0"]
impl crate::Resettable for D_ENTRY_PER_SLICE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
