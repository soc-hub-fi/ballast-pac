#[doc = "Register `CAM_CFG_SIZE` reader"]
pub struct R(crate::R<CAM_CFG_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CFG_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CFG_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CFG_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_CFG_SIZE` writer"]
pub struct W(crate::W<CAM_CFG_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CFG_SIZE_SPEC>;
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
impl From<crate::W<CAM_CFG_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CFG_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROWLEN` reader - Horizontal Resolution. It is used for slice mode. Value set into the bitfield must be equal to (rowlen-1)."]
pub type ROWLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ROWLEN` writer - Horizontal Resolution. It is used for slice mode. Value set into the bitfield must be equal to (rowlen-1)."]
pub type ROWLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_SIZE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Horizontal Resolution. It is used for slice mode. Value set into the bitfield must be equal to (rowlen-1)."]
    #[inline(always)]
    pub fn rowlen(&self) -> ROWLEN_R {
        ROWLEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Horizontal Resolution. It is used for slice mode. Value set into the bitfield must be equal to (rowlen-1)."]
    #[inline(always)]
    #[must_use]
    pub fn rowlen(&mut self) -> ROWLEN_W<16> {
        ROWLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal Resolution configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_cfg_size](index.html) module"]
pub struct CAM_CFG_SIZE_SPEC;
impl crate::RegisterSpec for CAM_CFG_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_cfg_size::R](R) reader structure"]
impl crate::Readable for CAM_CFG_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_cfg_size::W](W) writer structure"]
impl crate::Writable for CAM_CFG_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAM_CFG_SIZE to value 0"]
impl crate::Resettable for CAM_CFG_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
