#[doc = "Register `D_SRC_LINE_STRIDE` reader"]
pub struct R(crate::R<D_SRC_LINE_STRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_SRC_LINE_STRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_SRC_LINE_STRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_SRC_LINE_STRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_SRC_LINE_STRIDE` writer"]
pub struct W(crate::W<D_SRC_LINE_STRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_SRC_LINE_STRIDE_SPEC>;
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
impl From<crate::W<D_SRC_LINE_STRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_SRC_LINE_STRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC_LINE_STRIDE` reader - "]
pub struct SRC_LINE_STRIDE_R(crate::FieldReader<u32>);
impl SRC_LINE_STRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SRC_LINE_STRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_LINE_STRIDE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_LINE_STRIDE` writer - "]
pub struct SRC_LINE_STRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_LINE_STRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_line_stride(&self) -> SRC_LINE_STRIDE_R {
        SRC_LINE_STRIDE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_line_stride(&mut self) -> SRC_LINE_STRIDE_W {
        SRC_LINE_STRIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line stride of input cube\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_src_line_stride](index.html) module"]
pub struct D_SRC_LINE_STRIDE_SPEC;
impl crate::RegisterSpec for D_SRC_LINE_STRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_src_line_stride::R](R) reader structure"]
impl crate::Readable for D_SRC_LINE_STRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_src_line_stride::W](W) writer structure"]
impl crate::Writable for D_SRC_LINE_STRIDE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_SRC_LINE_STRIDE to value 0"]
impl crate::Resettable for D_SRC_LINE_STRIDE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}