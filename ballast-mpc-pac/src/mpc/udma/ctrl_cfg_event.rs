#[doc = "Register `CTRL_CFG_EVENT` reader"]
pub struct R(crate::R<CTRL_CFG_EVENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_CFG_EVENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_CFG_EVENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_CFG_EVENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_CFG_EVENT` writer"]
pub struct W(crate::W<CTRL_CFG_EVENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_CFG_EVENT_SPEC>;
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
impl From<crate::W<CTRL_CFG_EVENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_CFG_EVENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP_EVT0` reader - Forward event with ID matching CMP_EVT0 to peripherals as event0"]
pub struct CMP_EVT0_R(crate::FieldReader<u8, u8>);
impl CMP_EVT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMP_EVT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_EVT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_EVT0` writer - Forward event with ID matching CMP_EVT0 to peripherals as event0"]
pub struct CMP_EVT0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_EVT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CMP_EVT1` reader - Forward event with ID matching CMP_EVT1 to peripherals as event1"]
pub struct CMP_EVT1_R(crate::FieldReader<u8, u8>);
impl CMP_EVT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMP_EVT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_EVT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_EVT1` writer - Forward event with ID matching CMP_EVT1 to peripherals as event1"]
pub struct CMP_EVT1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_EVT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CMP_EVT2` reader - Forward event with ID matching CMP_EVT2 to peripherals as event2"]
pub struct CMP_EVT2_R(crate::FieldReader<u8, u8>);
impl CMP_EVT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMP_EVT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_EVT2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_EVT2` writer - Forward event with ID matching CMP_EVT2 to peripherals as event2"]
pub struct CMP_EVT2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_EVT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CMP_EVT3` reader - Forward event with ID matching CMP_EVT3 to peripherals as event3"]
pub struct CMP_EVT3_R(crate::FieldReader<u8, u8>);
impl CMP_EVT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMP_EVT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_EVT3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_EVT3` writer - Forward event with ID matching CMP_EVT3 to peripherals as event3"]
pub struct CMP_EVT3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_EVT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Forward event with ID matching CMP_EVT0 to peripherals as event0"]
    #[inline(always)]
    pub fn cmp_evt0(&self) -> CMP_EVT0_R {
        CMP_EVT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Forward event with ID matching CMP_EVT1 to peripherals as event1"]
    #[inline(always)]
    pub fn cmp_evt1(&self) -> CMP_EVT1_R {
        CMP_EVT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Forward event with ID matching CMP_EVT2 to peripherals as event2"]
    #[inline(always)]
    pub fn cmp_evt2(&self) -> CMP_EVT2_R {
        CMP_EVT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Forward event with ID matching CMP_EVT3 to peripherals as event3"]
    #[inline(always)]
    pub fn cmp_evt3(&self) -> CMP_EVT3_R {
        CMP_EVT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Forward event with ID matching CMP_EVT0 to peripherals as event0"]
    #[inline(always)]
    pub fn cmp_evt0(&mut self) -> CMP_EVT0_W {
        CMP_EVT0_W { w: self }
    }
    #[doc = "Bits 8:15 - Forward event with ID matching CMP_EVT1 to peripherals as event1"]
    #[inline(always)]
    pub fn cmp_evt1(&mut self) -> CMP_EVT1_W {
        CMP_EVT1_W { w: self }
    }
    #[doc = "Bits 16:23 - Forward event with ID matching CMP_EVT2 to peripherals as event2"]
    #[inline(always)]
    pub fn cmp_evt2(&mut self) -> CMP_EVT2_W {
        CMP_EVT2_W { w: self }
    }
    #[doc = "Bits 24:31 - Forward event with ID matching CMP_EVT3 to peripherals as event3"]
    #[inline(always)]
    pub fn cmp_evt3(&mut self) -> CMP_EVT3_W {
        CMP_EVT3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA peripherals external event configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_cfg_event](index.html) module"]
pub struct CTRL_CFG_EVENT_SPEC;
impl crate::RegisterSpec for CTRL_CFG_EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_cfg_event::R](R) reader structure"]
impl crate::Readable for CTRL_CFG_EVENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_cfg_event::W](W) writer structure"]
impl crate::Writable for CTRL_CFG_EVENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_CFG_EVENT to value 0"]
impl crate::Resettable for CTRL_CFG_EVENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
