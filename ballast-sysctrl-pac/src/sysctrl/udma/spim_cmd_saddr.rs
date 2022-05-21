#[doc = "Register `SPIM_CMD_SADDR` reader"]
pub struct R(crate::R<SPIM_CMD_SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIM_CMD_SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIM_CMD_SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIM_CMD_SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIM_CMD_SADDR` writer"]
pub struct W(crate::W<SPIM_CMD_SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIM_CMD_SADDR_SPEC>;
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
impl From<crate::W<SPIM_CMD_SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIM_CMD_SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_SADDR` reader - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub struct CMD_SADDR_R(crate::FieldReader<u32, u32>);
impl CMD_SADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CMD_SADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_SADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_SADDR` writer - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
pub struct CMD_SADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_SADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
    #[inline(always)]
    pub fn cmd_saddr(&self) -> CMD_SADDR_R {
        CMD_SADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure pointer to memory buffer: - Read: value of the pointer until transfer is over. Else returns 0 - Write: set Address Pointer to memory buffer start address"]
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
#[doc = "CMD SPI uDMA transfer address of associated buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spim_cmd_saddr](index.html) module"]
pub struct SPIM_CMD_SADDR_SPEC;
impl crate::RegisterSpec for SPIM_CMD_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spim_cmd_saddr::R](R) reader structure"]
impl crate::Readable for SPIM_CMD_SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spim_cmd_saddr::W](W) writer structure"]
impl crate::Writable for SPIM_CMD_SADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIM_CMD_SADDR to value 0"]
impl crate::Resettable for SPIM_CMD_SADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
