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
pub type FRAMESLICE_URX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMESLICE_URX` writer - Y coordinate of upper right corner of slice"]
pub type FRAMESLICE_URX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_UR_SPEC, u16, u16, 16, O>;
#[doc = "Field `FRAMESLICE_URY` reader - X coordinate of upper right corner of slice"]
pub type FRAMESLICE_URY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMESLICE_URY` writer - X coordinate of upper right corner of slice"]
pub type FRAMESLICE_URY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_UR_SPEC, u16, u16, 16, O>;
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
    #[must_use]
    pub fn frameslice_urx(&mut self) -> FRAMESLICE_URX_W<0> {
        FRAMESLICE_URX_W::new(self)
    }
    #[doc = "Bits 16:31 - X coordinate of upper right corner of slice"]
    #[inline(always)]
    #[must_use]
    pub fn frameslice_ury(&mut self) -> FRAMESLICE_URY_W<16> {
        FRAMESLICE_URY_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAM_CFG_UR to value 0"]
impl crate::Resettable for CAM_CFG_UR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
