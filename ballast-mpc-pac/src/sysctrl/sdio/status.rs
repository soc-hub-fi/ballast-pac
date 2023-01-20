#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOT` reader - "]
pub type EOT_R = crate::BitReader<bool>;
#[doc = "Field `EOT` writer - "]
pub type EOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `ERR` reader - "]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - "]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CARD_ERR` reader - "]
pub type CARD_ERR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CARD_ERR` writer - "]
pub type CARD_ERR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u16, u16, 10, O>;
#[doc = "Field `VHS` reader - Note: Is reset value correct?"]
pub type VHS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VHS` writer - Note: Is reset value correct?"]
pub type VHS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 4, O>;
#[doc = "Field `Status` reader - "]
pub type STATUS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Status` writer - "]
pub type STATUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn card_err(&self) -> CARD_ERR_R {
        CARD_ERR_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - Note: Is reset value correct?"]
    #[inline(always)]
    pub fn vhs(&self) -> VHS_R {
        VHS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EOT_W<0> {
        EOT_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<1> {
        ERR_W::new(self)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    #[must_use]
    pub fn card_err(&mut self) -> CARD_ERR_W<2> {
        CARD_ERR_W::new(self)
    }
    #[doc = "Bits 12:15 - Note: Is reset value correct?"]
    #[inline(always)]
    #[must_use]
    pub fn vhs(&mut self) -> VHS_W<12> {
        VHS_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<16> {
        STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x10"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
