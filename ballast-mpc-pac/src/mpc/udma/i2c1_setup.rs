#[doc = "Register `I2C1_SETUP` reader"]
pub struct R(crate::R<I2C1_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C1_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C1_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C1_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C1_SETUP` writer"]
pub struct W(crate::W<I2C1_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C1_SETUP_SPEC>;
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
impl From<crate::W<I2C1_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C1_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DO_RST` reader - Reset command used to abort the on-going transfer and clear busy and arbitration lost status flags."]
pub type DO_RST_R = crate::BitReader<bool>;
#[doc = "Field `DO_RST` writer - Reset command used to abort the on-going transfer and clear busy and arbitration lost status flags."]
pub type DO_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C1_SETUP_SPEC, bool, O>;
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
    #[must_use]
    pub fn do_rst(&mut self) -> DO_RST_W<0> {
        DO_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA I2C Configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c1_setup](index.html) module"]
pub struct I2C1_SETUP_SPEC;
impl crate::RegisterSpec for I2C1_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c1_setup::R](R) reader structure"]
impl crate::Readable for I2C1_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c1_setup::W](W) writer structure"]
impl crate::Writable for I2C1_SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C1_SETUP to value 0"]
impl crate::Resettable for I2C1_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
