#[doc = "Register `S_LUT_ACCESS_DATA` reader"]
pub struct R(crate::R<S_LUT_ACCESS_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_ACCESS_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_ACCESS_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_ACCESS_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_ACCESS_DATA` writer"]
pub struct W(crate::W<S_LUT_ACCESS_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_ACCESS_DATA_SPEC>;
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
impl From<crate::W<S_LUT_ACCESS_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_ACCESS_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_DATA` reader - "]
pub struct LUT_DATA_R(crate::FieldReader<u16>);
impl LUT_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LUT_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_DATA_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_DATA` writer - "]
pub struct LUT_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lut_data(&self) -> LUT_DATA_R {
        LUT_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lut_data(&mut self) -> LUT_DATA_W {
        LUT_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register of read or write LUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_access_data](index.html) module"]
pub struct S_LUT_ACCESS_DATA_SPEC;
impl crate::RegisterSpec for S_LUT_ACCESS_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_access_data::R](R) reader structure"]
impl crate::Readable for S_LUT_ACCESS_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_access_data::W](W) writer structure"]
impl crate::Writable for S_LUT_ACCESS_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S_LUT_ACCESS_DATA to value 0"]
impl crate::Resettable for S_LUT_ACCESS_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
