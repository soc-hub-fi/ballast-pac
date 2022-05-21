#[doc = "Register `D_DAIN_MAP` reader"]
pub struct R(crate::R<D_DAIN_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DAIN_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DAIN_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DAIN_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DAIN_MAP` writer"]
pub struct W(crate::W<D_DAIN_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DAIN_MAP_SPEC>;
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
impl From<crate::W<D_DAIN_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DAIN_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE_PACKED_A {
    #[doc = "0: `0`"]
    FALSE = 0,
    #[doc = "1: `1`"]
    TRUE = 1,
}
impl From<LINE_PACKED_A> for bool {
    #[inline(always)]
    fn from(variant: LINE_PACKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE_PACKED` reader - "]
pub struct LINE_PACKED_R(crate::FieldReader<bool, LINE_PACKED_A>);
impl LINE_PACKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINE_PACKED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE_PACKED_A {
        match self.bits {
            false => LINE_PACKED_A::FALSE,
            true => LINE_PACKED_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        **self == LINE_PACKED_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        **self == LINE_PACKED_A::TRUE
    }
}
impl core::ops::Deref for LINE_PACKED_R {
    type Target = crate::FieldReader<bool, LINE_PACKED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_PACKED` writer - "]
pub struct LINE_PACKED_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_PACKED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE_PACKED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(LINE_PACKED_A::FALSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(LINE_PACKED_A::TRUE)
    }
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SURF_PACKED_A {
    #[doc = "1: `1`"]
    TRUE = 1,
    #[doc = "0: `0`"]
    FALSE = 0,
}
impl From<SURF_PACKED_A> for bool {
    #[inline(always)]
    fn from(variant: SURF_PACKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SURF_PACKED` reader - "]
pub struct SURF_PACKED_R(crate::FieldReader<bool, SURF_PACKED_A>);
impl SURF_PACKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SURF_PACKED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SURF_PACKED_A {
        match self.bits {
            true => SURF_PACKED_A::TRUE,
            false => SURF_PACKED_A::FALSE,
        }
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        **self == SURF_PACKED_A::TRUE
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        **self == SURF_PACKED_A::FALSE
    }
}
impl core::ops::Deref for SURF_PACKED_R {
    type Target = crate::FieldReader<bool, SURF_PACKED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SURF_PACKED` writer - "]
pub struct SURF_PACKED_W<'a> {
    w: &'a mut W,
}
impl<'a> SURF_PACKED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SURF_PACKED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(SURF_PACKED_A::TRUE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(SURF_PACKED_A::FALSE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn line_packed(&self) -> LINE_PACKED_R {
        LINE_PACKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn surf_packed(&self) -> SURF_PACKED_R {
        SURF_PACKED_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn line_packed(&mut self) -> LINE_PACKED_W {
        LINE_PACKED_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn surf_packed(&mut self) -> SURF_PACKED_W {
        SURF_PACKED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Whether input cube is line packed or surface packed\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dain_map](index.html) module"]
pub struct D_DAIN_MAP_SPEC;
impl crate::RegisterSpec for D_DAIN_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dain_map::R](R) reader structure"]
impl crate::Readable for D_DAIN_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dain_map::W](W) writer structure"]
impl crate::Writable for D_DAIN_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_DAIN_MAP to value 0"]
impl crate::Resettable for D_DAIN_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
