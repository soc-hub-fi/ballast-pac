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
pub struct FRAMESLICE_LLX_R(crate::FieldReader<u16, u16>);
impl FRAMESLICE_LLX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRAMESLICE_LLX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMESLICE_LLX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMESLICE_LLX` writer - X coordinate of lower left corner of slice"]
pub struct FRAMESLICE_LLX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESLICE_LLX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `FRAMESLICE_LLY` reader - Y coordinate of lower left corner of slice"]
pub struct FRAMESLICE_LLY_R(crate::FieldReader<u16, u16>);
impl FRAMESLICE_LLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRAMESLICE_LLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMESLICE_LLY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMESLICE_LLY` writer - Y coordinate of lower left corner of slice"]
pub struct FRAMESLICE_LLY_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESLICE_LLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
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
    pub fn frameslice_llx(&mut self) -> FRAMESLICE_LLX_W {
        FRAMESLICE_LLX_W { w: self }
    }
    #[doc = "Bits 16:31 - Y coordinate of lower left corner of slice"]
    #[inline(always)]
    pub fn frameslice_lly(&mut self) -> FRAMESLICE_LLY_W {
        FRAMESLICE_LLY_W { w: self }
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
}
#[doc = "`reset()` method sets CAM_CFG_LL to value 0"]
impl crate::Resettable for CAM_CFG_LL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
