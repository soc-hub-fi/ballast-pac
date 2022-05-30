#[doc = "Register `PADDIR` reader"]
pub struct R(crate::R<PADDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADDIR` writer"]
pub struct W(crate::W<PADDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADDIR_SPEC>;
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
impl From<crate::W<PADDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADDIR` reader - "]
pub struct PADDIR_R(crate::FieldReader<u32>);
impl PADDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PADDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADDIR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADDIR` writer - "]
pub struct PADDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> PADDIR_W<'a> {
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
    pub fn paddir(&self) -> PADDIR_R {
        PADDIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn paddir(&mut self) -> PADDIR_W {
        PADDIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the direction of each of the GPIO pads. A value of 1 means it is configured as an output, while 0 configures it as an input.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paddir](index.html) module"]
pub struct PADDIR_SPEC;
impl crate::RegisterSpec for PADDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [paddir::R](R) reader structure"]
impl crate::Readable for PADDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paddir::W](W) writer structure"]
impl crate::Writable for PADDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADDIR to value 0"]
impl crate::Resettable for PADDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
