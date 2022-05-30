#[doc = "Register `START_LO` reader"]
pub struct R(crate::R<START_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<START_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<START_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<START_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `START_LO` writer"]
pub struct W(crate::W<START_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<START_LO_SPEC>;
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
impl From<crate::W<START_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<START_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_LO` reader - Timer high start command (sets EN in CFG_LO)"]
pub struct START_LO_R(crate::FieldReader<bool>);
impl START_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_LO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_LO` writer - Timer high start command (sets EN in CFG_LO)"]
pub struct START_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> START_LO_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Timer high start command (sets EN in CFG_LO)"]
    #[inline(always)]
    pub fn start_lo(&self) -> START_LO_R {
        START_LO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer high start command (sets EN in CFG_LO)"]
    #[inline(always)]
    pub fn start_lo(&mut self) -> START_LO_W {
        START_LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start Timer Low counting register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start_lo](index.html) module"]
pub struct START_LO_SPEC;
impl crate::RegisterSpec for START_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [start_lo::R](R) reader structure"]
impl crate::Readable for START_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [start_lo::W](W) writer structure"]
impl crate::Writable for START_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets START_LO to value 0"]
impl crate::Resettable for START_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
