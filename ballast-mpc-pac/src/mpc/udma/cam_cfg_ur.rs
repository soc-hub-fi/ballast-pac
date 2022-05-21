#[doc = "Register `CAM_CFG_UR` reader"]
pub struct R(crate::R<CAM_CFG_UR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CFG_UR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CFG_UR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CFG_UR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_CFG_UR` writer"]
pub struct W(crate::W<CAM_CFG_UR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CFG_UR_SPEC>;
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
impl From<crate::W<CAM_CFG_UR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CFG_UR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMESLICE_URX` reader - Y coordinate of upper right corner of slice"]
pub struct FRAMESLICE_URX_R(crate::FieldReader<u16, u16>);
impl FRAMESLICE_URX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRAMESLICE_URX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMESLICE_URX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMESLICE_URX` writer - Y coordinate of upper right corner of slice"]
pub struct FRAMESLICE_URX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESLICE_URX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `FRAMESLICE_URY` reader - X coordinate of upper right corner of slice"]
pub struct FRAMESLICE_URY_R(crate::FieldReader<u16, u16>);
impl FRAMESLICE_URY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRAMESLICE_URY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMESLICE_URY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMESLICE_URY` writer - X coordinate of upper right corner of slice"]
pub struct FRAMESLICE_URY_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESLICE_URY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Y coordinate of upper right corner of slice"]
    #[inline(always)]
    pub fn frameslice_urx(&self) -> FRAMESLICE_URX_R {
        FRAMESLICE_URX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - X coordinate of upper right corner of slice"]
    #[inline(always)]
    pub fn frameslice_ury(&self) -> FRAMESLICE_URY_R {
        FRAMESLICE_URY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Y coordinate of upper right corner of slice"]
    #[inline(always)]
    pub fn frameslice_urx(&mut self) -> FRAMESLICE_URX_W {
        FRAMESLICE_URX_W { w: self }
    }
    #[doc = "Bits 16:31 - X coordinate of upper right corner of slice"]
    #[inline(always)]
    pub fn frameslice_ury(&mut self) -> FRAMESLICE_URY_W {
        FRAMESLICE_URY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper Right corner configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_cfg_ur](index.html) module"]
pub struct CAM_CFG_UR_SPEC;
impl crate::RegisterSpec for CAM_CFG_UR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_cfg_ur::R](R) reader structure"]
impl crate::Readable for CAM_CFG_UR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_cfg_ur::W](W) writer structure"]
impl crate::Writable for CAM_CFG_UR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAM_CFG_UR to value 0"]
impl crate::Resettable for CAM_CFG_UR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
