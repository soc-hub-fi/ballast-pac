#[doc = "Register `I2C0_CMD_SIZE` reader"]
pub struct R(crate::R<I2C0_CMD_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_CMD_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_CMD_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_CMD_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C0_CMD_SIZE` writer"]
pub struct W(crate::W<I2C0_CMD_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_CMD_SIZE_SPEC>;
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
impl From<crate::W<I2C0_CMD_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_CMD_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_SIZE` reader - CMD buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub type CMD_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMD_SIZE` writer - CMD buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub type CMD_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C0_CMD_SIZE_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - CMD buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    pub fn cmd_size(&self) -> CMD_SIZE_R {
        CMD_SIZE_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CMD buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_size(&mut self) -> CMD_SIZE_W<0> {
        CMD_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA CMD I2C buffer size configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_cmd_size](index.html) module"]
pub struct I2C0_CMD_SIZE_SPEC;
impl crate::RegisterSpec for I2C0_CMD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_cmd_size::R](R) reader structure"]
impl crate::Readable for I2C0_CMD_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_cmd_size::W](W) writer structure"]
impl crate::Writable for I2C0_CMD_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C0_CMD_SIZE to value 0"]
impl crate::Resettable for I2C0_CMD_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
