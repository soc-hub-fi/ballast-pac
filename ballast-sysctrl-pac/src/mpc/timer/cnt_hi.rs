#[doc = "Register `CNT_HI` reader"]
pub struct R(crate::R<CNT_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_HI` writer"]
pub struct W(crate::W<CNT_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_HI_SPEC>;
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
impl From<crate::W<CNT_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_HI` reader - "]
pub struct CNT_HI_R(crate::FieldReader<u32, u32>);
impl CNT_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CNT_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_HI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_HI` writer - "]
pub struct CNT_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_HI_W<'a> {
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
    pub fn cnt_hi(&self) -> CNT_HI_R {
        CNT_HI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cnt_hi(&mut self) -> CNT_HI_W {
        CNT_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer High counter value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_hi](index.html) module"]
pub struct CNT_HI_SPEC;
impl crate::RegisterSpec for CNT_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_hi::R](R) reader structure"]
impl crate::Readable for CNT_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_hi::W](W) writer structure"]
impl crate::Writable for CNT_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT_HI to value 0"]
impl crate::Resettable for CNT_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
