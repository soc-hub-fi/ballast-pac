#[doc = "Register `CAM_VSYNC_POLARITY` reader"]
pub struct R(crate::R<CAM_VSYNC_POLARITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_VSYNC_POLARITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_VSYNC_POLARITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_VSYNC_POLARITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_VSYNC_POLARITY` writer"]
pub struct W(crate::W<CAM_VSYNC_POLARITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_VSYNC_POLARITY_SPEC>;
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
impl From<crate::W<CAM_VSYNC_POLARITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_VSYNC_POLARITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSYNC_POLARITY` reader - Set vsync polarity of CPI. - 1'b0: Active 0 - 1'b1: Active 1"]
pub struct VSYNC_POLARITY_R(crate::FieldReader<bool, bool>);
impl VSYNC_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VSYNC_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSYNC_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNC_POLARITY` writer - Set vsync polarity of CPI. - 1'b0: Active 0 - 1'b1: Active 1"]
pub struct VSYNC_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_POLARITY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set vsync polarity of CPI. - 1'b0: Active 0 - 1'b1: Active 1"]
    #[inline(always)]
    pub fn vsync_polarity(&self) -> VSYNC_POLARITY_R {
        VSYNC_POLARITY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set vsync polarity of CPI. - 1'b0: Active 0 - 1'b1: Active 1"]
    #[inline(always)]
    pub fn vsync_polarity(&mut self) -> VSYNC_POLARITY_W {
        VSYNC_POLARITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VSYNC Polarity register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_vsync_polarity](index.html) module"]
pub struct CAM_VSYNC_POLARITY_SPEC;
impl crate::RegisterSpec for CAM_VSYNC_POLARITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_vsync_polarity::R](R) reader structure"]
impl crate::Readable for CAM_VSYNC_POLARITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_vsync_polarity::W](W) writer structure"]
impl crate::Writable for CAM_VSYNC_POLARITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAM_VSYNC_POLARITY to value 0"]
impl crate::Resettable for CAM_VSYNC_POLARITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
