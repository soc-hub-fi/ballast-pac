#[doc = "Register `I2C0_SETUP` reader"]
pub struct R(crate::R<I2C0_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C0_SETUP` writer"]
pub struct W(crate::W<I2C0_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_SETUP_SPEC>;
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
impl From<crate::W<I2C0_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DO_RST` reader - Reset command used to abort the on-going transfer and clear busy and arbitration lost status flags."]
pub struct DO_RST_R(crate::FieldReader<bool>);
impl DO_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DO_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DO_RST` writer - Reset command used to abort the on-going transfer and clear busy and arbitration lost status flags."]
pub struct DO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_RST_W<'a> {
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
    #[doc = "Bit 0 - Reset command used to abort the on-going transfer and clear busy and arbitration lost status flags."]
    #[inline(always)]
    pub fn do_rst(&self) -> DO_RST_R {
        DO_RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset command used to abort the on-going transfer and clear busy and arbitration lost status flags."]
    #[inline(always)]
    pub fn do_rst(&mut self) -> DO_RST_W {
        DO_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA I2C Configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_setup](index.html) module"]
pub struct I2C0_SETUP_SPEC;
impl crate::RegisterSpec for I2C0_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_setup::R](R) reader structure"]
impl crate::Readable for I2C0_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_setup::W](W) writer structure"]
impl crate::Writable for I2C0_SETUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C0_SETUP to value 0"]
impl crate::Resettable for I2C0_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
