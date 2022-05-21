#[doc = "Register `refclk` reader"]
pub struct R(crate::R<REFCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `refclk` writer"]
pub struct W(crate::W<REFCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCLK_SPEC>;
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
impl From<crate::W<REFCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `drive_strength` reader - "]
pub struct DRIVE_STRENGTH_R(crate::FieldReader<u8, u8>);
impl DRIVE_STRENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_STRENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_STRENGTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `drive_strength` writer - "]
pub struct DRIVE_STRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_STRENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u8 & 7);
        self.w
    }
}
#[doc = "Field `enable` reader - "]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable` writer - "]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u8 & 1) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn drive_strength(&self) -> DRIVE_STRENGTH_R {
        DRIVE_STRENGTH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn drive_strength(&mut self) -> DRIVE_STRENGTH_W {
        DRIVE_STRENGTH_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refclk](index.html) module"]
pub struct REFCLK_SPEC;
impl crate::RegisterSpec for REFCLK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [refclk::R](R) reader structure"]
impl crate::Readable for REFCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refclk::W](W) writer structure"]
impl crate::Writable for REFCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets refclk to value 0x0f"]
impl crate::Resettable for REFCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
