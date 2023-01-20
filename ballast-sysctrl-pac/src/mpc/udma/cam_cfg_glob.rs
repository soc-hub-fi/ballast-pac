#[doc = "Register `CAM_CFG_GLOB` reader"]
pub struct R(crate::R<CAM_CFG_GLOB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CFG_GLOB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CFG_GLOB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CFG_GLOB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_CFG_GLOB` writer"]
pub struct W(crate::W<CAM_CFG_GLOB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CFG_GLOB_SPEC>;
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
impl From<crate::W<CAM_CFG_GLOB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CFG_GLOB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEDROP_EN` reader - Frame dropping:"]
pub type FRAMEDROP_EN_R = crate::BitReader<bool>;
#[doc = "Field `FRAMEDROP_EN` writer - Frame dropping:"]
pub type FRAMEDROP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAM_CFG_GLOB_SPEC, bool, O>;
#[doc = "Field `FRAMEDROP_VAL` reader - ) Sets how many frames should be dropped after each received"]
pub type FRAMEDROP_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAMEDROP_VAL` writer - ) Sets how many frames should be dropped after each received"]
pub type FRAMEDROP_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_GLOB_SPEC, u8, u8, 6, O>;
#[doc = "Field `FRAMESLICE_EN` reader - nput frame slicing: - 1'b0: disable - 1'b1: enabl"]
pub type FRAMESLICE_EN_R = crate::BitReader<bool>;
#[doc = "Field `FRAMESLICE_EN` writer - nput frame slicing: - 1'b0: disable - 1'b1: enabl"]
pub type FRAMESLICE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAM_CFG_GLOB_SPEC, bool, O>;
#[doc = "Field `FORMAT` reader - Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND"]
pub type FORMAT_R = crate::FieldReader<u8, FORMAT_A>;
#[doc = "Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: `0`"]
    RGB565 = 0,
    #[doc = "1: `1`"]
    RGB555 = 1,
    #[doc = "2: `10`"]
    RGB444 = 2,
    #[doc = "4: `100`"]
    BYPASS_LITEND = 4,
    #[doc = "5: `101`"]
    BYPASS_BIGEND = 5,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMAT_A> {
        match self.bits {
            0 => Some(FORMAT_A::RGB565),
            1 => Some(FORMAT_A::RGB555),
            2 => Some(FORMAT_A::RGB444),
            4 => Some(FORMAT_A::BYPASS_LITEND),
            5 => Some(FORMAT_A::BYPASS_BIGEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == FORMAT_A::RGB565
    }
    #[doc = "Checks if the value of the field is `RGB555`"]
    #[inline(always)]
    pub fn is_rgb555(&self) -> bool {
        *self == FORMAT_A::RGB555
    }
    #[doc = "Checks if the value of the field is `RGB444`"]
    #[inline(always)]
    pub fn is_rgb444(&self) -> bool {
        *self == FORMAT_A::RGB444
    }
    #[doc = "Checks if the value of the field is `BYPASS_LITEND`"]
    #[inline(always)]
    pub fn is_bypass_litend(&self) -> bool {
        *self == FORMAT_A::BYPASS_LITEND
    }
    #[doc = "Checks if the value of the field is `BYPASS_BIGEND`"]
    #[inline(always)]
    pub fn is_bypass_bigend(&self) -> bool {
        *self == FORMAT_A::BYPASS_BIGEND
    }
}
#[doc = "Field `FORMAT` writer - Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND"]
pub type FORMAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_GLOB_SPEC, u8, FORMAT_A, 3, O>;
impl<'a, const O: u8> FORMAT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB565)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rgb555(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB555)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rgb444(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB444)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn bypass_litend(self) -> &'a mut W {
        self.variant(FORMAT_A::BYPASS_LITEND)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn bypass_bigend(self) -> &'a mut W {
        self.variant(FORMAT_A::BYPASS_BIGEND)
    }
}
#[doc = "Field `SHIFT` reader - Right shift of final pixel value (DivFactor) NOTE: not used if FORMAT == BYPASS"]
pub type SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHIFT` writer - Right shift of final pixel value (DivFactor) NOTE: not used if FORMAT == BYPASS"]
pub type SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAM_CFG_GLOB_SPEC, u8, u8, 4, O>;
#[doc = "Field `EN` reader - Enable data rx from camera interface. The enable/disable happens only at the start of a frame. - 1'b0: disable - 1'b1: enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable data rx from camera interface. The enable/disable happens only at the start of a frame. - 1'b0: disable - 1'b1: enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAM_CFG_GLOB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Frame dropping:"]
    #[inline(always)]
    pub fn framedrop_en(&self) -> FRAMEDROP_EN_R {
        FRAMEDROP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - ) Sets how many frames should be dropped after each received"]
    #[inline(always)]
    pub fn framedrop_val(&self) -> FRAMEDROP_VAL_R {
        FRAMEDROP_VAL_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - nput frame slicing: - 1'b0: disable - 1'b1: enabl"]
    #[inline(always)]
    pub fn frameslice_en(&self) -> FRAMESLICE_EN_R {
        FRAMESLICE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:14 - Right shift of final pixel value (DivFactor) NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Enable data rx from camera interface. The enable/disable happens only at the start of a frame. - 1'b0: disable - 1'b1: enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame dropping:"]
    #[inline(always)]
    #[must_use]
    pub fn framedrop_en(&mut self) -> FRAMEDROP_EN_W<0> {
        FRAMEDROP_EN_W::new(self)
    }
    #[doc = "Bits 1:6 - ) Sets how many frames should be dropped after each received"]
    #[inline(always)]
    #[must_use]
    pub fn framedrop_val(&mut self) -> FRAMEDROP_VAL_W<1> {
        FRAMEDROP_VAL_W::new(self)
    }
    #[doc = "Bit 7 - nput frame slicing: - 1'b0: disable - 1'b1: enabl"]
    #[inline(always)]
    #[must_use]
    pub fn frameslice_en(&mut self) -> FRAMESLICE_EN_W<7> {
        FRAMESLICE_EN_W::new(self)
    }
    #[doc = "Bits 8:10 - Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<8> {
        FORMAT_W::new(self)
    }
    #[doc = "Bits 11:14 - Right shift of final pixel value (DivFactor) NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    #[must_use]
    pub fn shift(&mut self) -> SHIFT_W<11> {
        SHIFT_W::new(self)
    }
    #[doc = "Bit 31 - Enable data rx from camera interface. The enable/disable happens only at the start of a frame. - 1'b0: disable - 1'b1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_cfg_glob](index.html) module"]
pub struct CAM_CFG_GLOB_SPEC;
impl crate::RegisterSpec for CAM_CFG_GLOB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_cfg_glob::R](R) reader structure"]
impl crate::Readable for CAM_CFG_GLOB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_cfg_glob::W](W) writer structure"]
impl crate::Writable for CAM_CFG_GLOB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAM_CFG_GLOB to value 0"]
impl crate::Resettable for CAM_CFG_GLOB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
