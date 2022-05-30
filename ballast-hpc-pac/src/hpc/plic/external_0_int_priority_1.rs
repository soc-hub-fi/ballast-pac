#[doc = "Register `external_0_int_priority_1` reader"]
pub struct R(crate::R<EXTERNAL_0_INT_PRIORITY_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTERNAL_0_INT_PRIORITY_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTERNAL_0_INT_PRIORITY_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTERNAL_0_INT_PRIORITY_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `external_0_int_priority_1` writer"]
pub struct W(crate::W<EXTERNAL_0_INT_PRIORITY_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTERNAL_0_INT_PRIORITY_1_SPEC>;
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
impl From<crate::W<EXTERNAL_0_INT_PRIORITY_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTERNAL_0_INT_PRIORITY_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `external_0_int_priority_1` reader - "]
pub struct EXTERNAL_0_INT_PRIORITY_1_R(crate::FieldReader<u32>);
impl EXTERNAL_0_INT_PRIORITY_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EXTERNAL_0_INT_PRIORITY_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTERNAL_0_INT_PRIORITY_1_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `external_0_int_priority_1` writer - "]
pub struct EXTERNAL_0_INT_PRIORITY_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_0_INT_PRIORITY_1_W<'a> {
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
    pub fn external_0_int_priority_1(&self) -> EXTERNAL_0_INT_PRIORITY_1_R {
        EXTERNAL_0_INT_PRIORITY_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn external_0_int_priority_1(&mut self) -> EXTERNAL_0_INT_PRIORITY_1_W {
        EXTERNAL_0_INT_PRIORITY_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [external_0_int_priority_1](index.html) module"]
pub struct EXTERNAL_0_INT_PRIORITY_1_SPEC;
impl crate::RegisterSpec for EXTERNAL_0_INT_PRIORITY_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [external_0_int_priority_1::R](R) reader structure"]
impl crate::Readable for EXTERNAL_0_INT_PRIORITY_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [external_0_int_priority_1::W](W) writer structure"]
impl crate::Writable for EXTERNAL_0_INT_PRIORITY_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets external_0_int_priority_1 to value 0"]
impl crate::Resettable for EXTERNAL_0_INT_PRIORITY_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
