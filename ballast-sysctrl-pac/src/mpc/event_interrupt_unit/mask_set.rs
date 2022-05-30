#[doc = "Register `MASK_set` reader"]
pub struct R(crate::R<MASK_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK_set` writer"]
pub struct W(crate::W<MASK_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SET_SPEC>;
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
impl From<crate::W<MASK_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_set` reader - "]
pub struct MASK_SET_R(crate::FieldReader<u32>);
impl MASK_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MASK_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_SET_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_set` writer - "]
pub struct MASK_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_SET_W<'a> {
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
    pub fn mask_set(&self) -> MASK_SET_R {
        MASK_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mask_set(&mut self) -> MASK_SET_W {
        MASK_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the MASK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_9004 sets the bits of the MASK register selected. Writing to 0x1A10_9008 clears the bits of the MASK register selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask_set](index.html) module"]
pub struct MASK_SET_SPEC;
impl crate::RegisterSpec for MASK_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask_set::R](R) reader structure"]
impl crate::Readable for MASK_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask_set::W](W) writer structure"]
impl crate::Writable for MASK_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK_set to value 0"]
impl crate::Resettable for MASK_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
