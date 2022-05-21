#[doc = "Register `msip` reader"]
pub struct R(crate::R<MSIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `msip` writer"]
pub struct W(crate::W<MSIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSIP_SPEC>;
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
impl From<crate::W<MSIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `msip_0` reader - "]
pub struct MSIP_0_R(crate::FieldReader<bool, bool>);
impl MSIP_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSIP_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSIP_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `msip_0` writer - "]
pub struct MSIP_0_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIP_0_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u64 & 1);
        self.w
    }
}
#[doc = "Field `msip_1` reader - "]
pub struct MSIP_1_R(crate::FieldReader<bool, bool>);
impl MSIP_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSIP_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSIP_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `msip_1` writer - "]
pub struct MSIP_1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIP_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u64 & 1) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn msip_0(&self) -> MSIP_0_R {
        MSIP_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn msip_1(&self) -> MSIP_1_R {
        MSIP_1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn msip_0(&mut self) -> MSIP_0_W {
        MSIP_0_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn msip_1(&mut self) -> MSIP_1_W {
        MSIP_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Machine mode software interrupt (IPI)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msip](index.html) module"]
pub struct MSIP_SPEC;
impl crate::RegisterSpec for MSIP_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [msip::R](R) reader structure"]
impl crate::Readable for MSIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msip::W](W) writer structure"]
impl crate::Writable for MSIP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets msip to value 0"]
impl crate::Resettable for MSIP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
