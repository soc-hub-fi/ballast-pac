#[doc = "Register `SPIM_CMD_SIZE` reader"]
pub struct R(crate::R<SPIM_CMD_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIM_CMD_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIM_CMD_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIM_CMD_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIM_CMD_SIZE` writer"]
pub struct W(crate::W<SPIM_CMD_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIM_CMD_SIZE_SPEC>;
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
impl From<crate::W<SPIM_CMD_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIM_CMD_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_SIZE` reader - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
pub struct CMD_SIZE_R(crate::FieldReader<u32>);
impl CMD_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CMD_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_SIZE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_SIZE` writer - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
pub struct CMD_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
    #[inline(always)]
    pub fn cmd_size(&self) -> CMD_SIZE_R {
        CMD_SIZE_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Buffer size in bytes. (1MBytes maximum) - Read: buffer size left - Write: set buffer size"]
    #[inline(always)]
    pub fn cmd_size(&mut self) -> CMD_SIZE_W {
        CMD_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMD SPI uDMA transfer size of buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spim_cmd_size](index.html) module"]
pub struct SPIM_CMD_SIZE_SPEC;
impl crate::RegisterSpec for SPIM_CMD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spim_cmd_size::R](R) reader structure"]
impl crate::Readable for SPIM_CMD_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spim_cmd_size::W](W) writer structure"]
impl crate::Writable for SPIM_CMD_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIM_CMD_SIZE to value 0"]
impl crate::Resettable for SPIM_CMD_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
