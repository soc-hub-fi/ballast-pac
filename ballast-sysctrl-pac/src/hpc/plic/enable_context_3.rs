#[doc = "Register `enable_context_3` reader"]
pub struct R(crate::R<ENABLE_CONTEXT_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_CONTEXT_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_CONTEXT_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_CONTEXT_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `enable_context_3` writer"]
pub struct W(crate::W<ENABLE_CONTEXT_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_CONTEXT_3_SPEC>;
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
impl From<crate::W<ENABLE_CONTEXT_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_CONTEXT_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer0_int0_enable` reader - "]
pub struct TIMER0_INT0_ENABLE_R(crate::FieldReader<bool>);
impl TIMER0_INT0_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_INT0_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_INT0_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer0_int0_enable` writer - "]
pub struct TIMER0_INT0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_INT0_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `timer0_int1_enable` reader - "]
pub struct TIMER0_INT1_ENABLE_R(crate::FieldReader<bool>);
impl TIMER0_INT1_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_INT1_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_INT1_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer0_int1_enable` writer - "]
pub struct TIMER0_INT1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_INT1_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `timer1_int0_enable` reader - "]
pub struct TIMER1_INT0_ENABLE_R(crate::FieldReader<bool>);
impl TIMER1_INT0_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_INT0_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_INT0_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer1_int0_enable` writer - "]
pub struct TIMER1_INT0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_INT0_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `timer1_int1_enable` reader - "]
pub struct TIMER1_INT1_ENABLE_R(crate::FieldReader<bool>);
impl TIMER1_INT1_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_INT1_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_INT1_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer1_int1_enable` writer - "]
pub struct TIMER1_INT1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_INT1_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `external_int0_enable` reader - "]
pub struct EXTERNAL_INT0_ENABLE_R(crate::FieldReader<bool>);
impl EXTERNAL_INT0_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTERNAL_INT0_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTERNAL_INT0_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `external_int0_enable` writer - "]
pub struct EXTERNAL_INT0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_INT0_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `external_int1_enable` reader - "]
pub struct EXTERNAL_INT1_ENABLE_R(crate::FieldReader<bool>);
impl EXTERNAL_INT1_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTERNAL_INT1_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTERNAL_INT1_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `external_int1_enable` writer - "]
pub struct EXTERNAL_INT1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_INT1_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_int0_enable(&self) -> TIMER0_INT0_ENABLE_R {
        TIMER0_INT0_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer0_int1_enable(&self) -> TIMER0_INT1_ENABLE_R {
        TIMER0_INT1_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer1_int0_enable(&self) -> TIMER1_INT0_ENABLE_R {
        TIMER1_INT0_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer1_int1_enable(&self) -> TIMER1_INT1_ENABLE_R {
        TIMER1_INT1_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn external_int0_enable(&self) -> EXTERNAL_INT0_ENABLE_R {
        EXTERNAL_INT0_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn external_int1_enable(&self) -> EXTERNAL_INT1_ENABLE_R {
        EXTERNAL_INT1_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_int0_enable(&mut self) -> TIMER0_INT0_ENABLE_W {
        TIMER0_INT0_ENABLE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer0_int1_enable(&mut self) -> TIMER0_INT1_ENABLE_W {
        TIMER0_INT1_ENABLE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer1_int0_enable(&mut self) -> TIMER1_INT0_ENABLE_W {
        TIMER1_INT0_ENABLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer1_int1_enable(&mut self) -> TIMER1_INT1_ENABLE_W {
        TIMER1_INT1_ENABLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn external_int0_enable(&mut self) -> EXTERNAL_INT0_ENABLE_W {
        EXTERNAL_INT0_ENABLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn external_int1_enable(&mut self) -> EXTERNAL_INT1_ENABLE_W {
        EXTERNAL_INT1_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_context_3](index.html) module"]
pub struct ENABLE_CONTEXT_3_SPEC;
impl crate::RegisterSpec for ENABLE_CONTEXT_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable_context_3::R](R) reader structure"]
impl crate::Readable for ENABLE_CONTEXT_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable_context_3::W](W) writer structure"]
impl crate::Writable for ENABLE_CONTEXT_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets enable_context_3 to value 0"]
impl crate::Resettable for ENABLE_CONTEXT_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
