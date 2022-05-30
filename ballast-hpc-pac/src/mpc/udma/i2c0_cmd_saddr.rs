#[doc = "Register `I2C0_CMD_SADDR` reader"]
pub struct R(crate::R<I2C0_CMD_SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_CMD_SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_CMD_SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_CMD_SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C0_CMD_SADDR` writer"]
pub struct W(crate::W<I2C0_CMD_SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_CMD_SADDR_SPEC>;
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
impl From<crate::W<I2C0_CMD_SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_CMD_SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_SADDR` reader - CMD buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
pub struct CMD_SADDR_R(crate::FieldReader<u32>);
impl CMD_SADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CMD_SADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_SADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_SADDR` writer - CMD buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
pub struct CMD_SADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_SADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | (value as u32 & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - CMD buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
    #[inline(always)]
    pub fn cmd_saddr(&self) -> CMD_SADDR_R {
        CMD_SADDR_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - CMD buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
    #[inline(always)]
    pub fn cmd_saddr(&mut self) -> CMD_SADDR_W {
        CMD_SADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA CMD I2C buffer base address configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_cmd_saddr](index.html) module"]
pub struct I2C0_CMD_SADDR_SPEC;
impl crate::RegisterSpec for I2C0_CMD_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_cmd_saddr::R](R) reader structure"]
impl crate::Readable for I2C0_CMD_SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_cmd_saddr::W](W) writer structure"]
impl crate::Writable for I2C0_CMD_SADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C0_CMD_SADDR to value 0"]
impl crate::Resettable for I2C0_CMD_SADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
