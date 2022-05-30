#[doc = "Register `PADOUT_32_63` reader"]
pub struct R(crate::R<PADOUT_32_63_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADOUT_32_63_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADOUT_32_63_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADOUT_32_63_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADOUT_32_63` writer"]
pub struct W(crate::W<PADOUT_32_63_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADOUT_32_63_SPEC>;
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
impl From<crate::W<PADOUT_32_63_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADOUT_32_63_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_OUT` reader - "]
pub struct DATA_OUT_R(crate::FieldReader<u32>);
impl DATA_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_OUT_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_OUT` writer - "]
pub struct DATA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_OUT_W<'a> {
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
    pub fn data_out(&self) -> DATA_OUT_R {
        DATA_OUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_out(&mut self) -> DATA_OUT_W {
        DATA_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 DATA_OUT (R/W) GPIO\\[63:32\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padout_32_63](index.html) module"]
pub struct PADOUT_32_63_SPEC;
impl crate::RegisterSpec for PADOUT_32_63_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padout_32_63::R](R) reader structure"]
impl crate::Readable for PADOUT_32_63_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padout_32_63::W](W) writer structure"]
impl crate::Writable for PADOUT_32_63_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADOUT_32_63 to value 0"]
impl crate::Resettable for PADOUT_32_63_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
