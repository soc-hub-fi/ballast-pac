#[doc = "Register `D_PARTIAL_WIDTH_IN` reader"]
pub struct R(crate::R<D_PARTIAL_WIDTH_IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PARTIAL_WIDTH_IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PARTIAL_WIDTH_IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PARTIAL_WIDTH_IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_PARTIAL_WIDTH_IN` writer"]
pub struct W(crate::W<D_PARTIAL_WIDTH_IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_PARTIAL_WIDTH_IN_SPEC>;
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
impl From<crate::W<D_PARTIAL_WIDTH_IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_PARTIAL_WIDTH_IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARTIAL_WIDTH_IN_FIRST` reader - "]
pub struct PARTIAL_WIDTH_IN_FIRST_R(crate::FieldReader<u16>);
impl PARTIAL_WIDTH_IN_FIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PARTIAL_WIDTH_IN_FIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTIAL_WIDTH_IN_FIRST_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTIAL_WIDTH_IN_FIRST` writer - "]
pub struct PARTIAL_WIDTH_IN_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PARTIAL_WIDTH_IN_FIRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `PARTIAL_WIDTH_IN_LAST` reader - "]
pub struct PARTIAL_WIDTH_IN_LAST_R(crate::FieldReader<u16>);
impl PARTIAL_WIDTH_IN_LAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PARTIAL_WIDTH_IN_LAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTIAL_WIDTH_IN_LAST_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTIAL_WIDTH_IN_LAST` writer - "]
pub struct PARTIAL_WIDTH_IN_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> PARTIAL_WIDTH_IN_LAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `PARTIAL_WIDTH_IN_MID` reader - "]
pub struct PARTIAL_WIDTH_IN_MID_R(crate::FieldReader<u16>);
impl PARTIAL_WIDTH_IN_MID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PARTIAL_WIDTH_IN_MID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTIAL_WIDTH_IN_MID_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTIAL_WIDTH_IN_MID` writer - "]
pub struct PARTIAL_WIDTH_IN_MID_W<'a> {
    w: &'a mut W,
}
impl<'a> PARTIAL_WIDTH_IN_MID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn partial_width_in_first(&self) -> PARTIAL_WIDTH_IN_FIRST_R {
        PARTIAL_WIDTH_IN_FIRST_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn partial_width_in_last(&self) -> PARTIAL_WIDTH_IN_LAST_R {
        PARTIAL_WIDTH_IN_LAST_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn partial_width_in_mid(&self) -> PARTIAL_WIDTH_IN_MID_R {
        PARTIAL_WIDTH_IN_MID_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn partial_width_in_first(&mut self) -> PARTIAL_WIDTH_IN_FIRST_W {
        PARTIAL_WIDTH_IN_FIRST_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn partial_width_in_last(&mut self) -> PARTIAL_WIDTH_IN_LAST_W {
        PARTIAL_WIDTH_IN_LAST_W { w: self }
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn partial_width_in_mid(&mut self) -> PARTIAL_WIDTH_IN_MID_W {
        PARTIAL_WIDTH_IN_MID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_partial_width_in](index.html) module"]
pub struct D_PARTIAL_WIDTH_IN_SPEC;
impl crate::RegisterSpec for D_PARTIAL_WIDTH_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_partial_width_in::R](R) reader structure"]
impl crate::Readable for D_PARTIAL_WIDTH_IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_partial_width_in::W](W) writer structure"]
impl crate::Writable for D_PARTIAL_WIDTH_IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_PARTIAL_WIDTH_IN to value 0"]
impl crate::Resettable for D_PARTIAL_WIDTH_IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
