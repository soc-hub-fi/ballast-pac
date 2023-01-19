#[doc = "Register `D_PARTIAL_WIDTH_OUT` reader"]
pub struct R(crate::R<D_PARTIAL_WIDTH_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PARTIAL_WIDTH_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PARTIAL_WIDTH_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PARTIAL_WIDTH_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_PARTIAL_WIDTH_OUT` writer"]
pub struct W(crate::W<D_PARTIAL_WIDTH_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_PARTIAL_WIDTH_OUT_SPEC>;
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
impl From<crate::W<D_PARTIAL_WIDTH_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_PARTIAL_WIDTH_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARTIAL_WIDTH_OUT_FIRST` reader - "]
pub type PARTIAL_WIDTH_OUT_FIRST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PARTIAL_WIDTH_OUT_FIRST` writer - "]
pub type PARTIAL_WIDTH_OUT_FIRST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_PARTIAL_WIDTH_OUT_SPEC, u16, u16, 10, O>;
#[doc = "Field `PARTIAL_WIDTH_OUT_LAST` reader - "]
pub type PARTIAL_WIDTH_OUT_LAST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PARTIAL_WIDTH_OUT_LAST` writer - "]
pub type PARTIAL_WIDTH_OUT_LAST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_PARTIAL_WIDTH_OUT_SPEC, u16, u16, 10, O>;
#[doc = "Field `PARTIAL_WIDTH_OUT_MID` reader - "]
pub type PARTIAL_WIDTH_OUT_MID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PARTIAL_WIDTH_OUT_MID` writer - "]
pub type PARTIAL_WIDTH_OUT_MID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_PARTIAL_WIDTH_OUT_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn partial_width_out_first(&self) -> PARTIAL_WIDTH_OUT_FIRST_R {
        PARTIAL_WIDTH_OUT_FIRST_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn partial_width_out_last(&self) -> PARTIAL_WIDTH_OUT_LAST_R {
        PARTIAL_WIDTH_OUT_LAST_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn partial_width_out_mid(&self) -> PARTIAL_WIDTH_OUT_MID_R {
        PARTIAL_WIDTH_OUT_MID_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn partial_width_out_first(&mut self) -> PARTIAL_WIDTH_OUT_FIRST_W<0> {
        PARTIAL_WIDTH_OUT_FIRST_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn partial_width_out_last(&mut self) -> PARTIAL_WIDTH_OUT_LAST_W<10> {
        PARTIAL_WIDTH_OUT_LAST_W::new(self)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    #[must_use]
    pub fn partial_width_out_mid(&mut self) -> PARTIAL_WIDTH_OUT_MID_W<20> {
        PARTIAL_WIDTH_OUT_MID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Partial width for first, last and middle partitions of output cube\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_partial_width_out](index.html) module"]
pub struct D_PARTIAL_WIDTH_OUT_SPEC;
impl crate::RegisterSpec for D_PARTIAL_WIDTH_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_partial_width_out::R](R) reader structure"]
impl crate::Readable for D_PARTIAL_WIDTH_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_partial_width_out::W](W) writer structure"]
impl crate::Writable for D_PARTIAL_WIDTH_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_PARTIAL_WIDTH_OUT to value 0"]
impl crate::Resettable for D_PARTIAL_WIDTH_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
