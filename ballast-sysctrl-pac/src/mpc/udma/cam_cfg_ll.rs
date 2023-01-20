#[doc = "Register `CAM_CFG_LL` reader"]
pub struct R(crate::R<CAM_CFG_LL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CFG_LL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CFG_LL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CFG_LL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_CFG_LL` writer"]
pub struct W(crate::W<CAM_CFG_LL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CFG_LL_SPEC>;
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
impl From<crate::W<CAM_CFG_LL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CFG_LL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMESLICE_LLX` reader - X coordinate of lower left corner of slice"]
pub type FRAMESLICE_LLX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMESLICE_LLX` writer - X coordinate of lower left corner of slice"]
pub type FRAMESLICE_LLX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_LL_SPEC, u16, u16, 16, O>;
#[doc = "Field `FRAMESLICE_LLY` reader - Y coordinate of lower left corner of slice"]
pub type FRAMESLICE_LLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMESLICE_LLY` writer - Y coordinate of lower left corner of slice"]
pub type FRAMESLICE_LLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_LL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - X coordinate of lower left corner of slice"]
    #[inline(always)]
    pub fn frameslice_llx(&self) -> FRAMESLICE_LLX_R {
        FRAMESLICE_LLX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Y coordinate of lower left corner of slice"]
    #[inline(always)]
    pub fn frameslice_lly(&self) -> FRAMESLICE_LLY_R {
        FRAMESLICE_LLY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - X coordinate of lower left corner of slice"]
    #[inline(always)]
    #[must_use]
    pub fn frameslice_llx(&mut self) -> FRAMESLICE_LLX_W<0> {
        FRAMESLICE_LLX_W::new(self)
    }
    #[doc = "Bits 16:31 - Y coordinate of lower left corner of slice"]
    #[inline(always)]
    #[must_use]
    pub fn frameslice_lly(&mut self) -> FRAMESLICE_LLY_W<16> {
        FRAMESLICE_LLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower Left corner configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_cfg_ll](index.html) module"]
pub struct CAM_CFG_LL_SPEC;
impl crate::RegisterSpec for CAM_CFG_LL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_cfg_ll::R](R) reader structure"]
impl crate::Readable for CAM_CFG_LL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_cfg_ll::W](W) writer structure"]
impl crate::Writable for CAM_CFG_LL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAM_CFG_LL to value 0"]
impl crate::Resettable for CAM_CFG_LL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
