#[doc = "Register `RX_CH_CFG` reader"]
pub struct R(crate::R<RX_CH_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CH_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CH_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CH_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CH_CFG` writer"]
pub struct W(crate::W<RX_CH_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CH_CFG_SPEC>;
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
impl From<crate::W<RX_CH_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CH_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE` reader - Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit"]
pub type SIZE_R = crate::FieldReader<u8, SIZE_A>;
#[doc = "Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: `0`"]
    _8BITS = 0,
    #[doc = "1: `1`"]
    _16BITS = 1,
    #[doc = "2: `10`"]
    _32BITS = 2,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            0 => Some(SIZE_A::_8BITS),
            1 => Some(SIZE_A::_16BITS),
            2 => Some(SIZE_A::_32BITS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == SIZE_A::_8BITS
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        *self == SIZE_A::_16BITS
    }
    #[doc = "Checks if the value of the field is `_32BITS`"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == SIZE_A::_32BITS
    }
}
#[doc = "Field `SIZE` writer - Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_CH_CFG_SPEC, u8, SIZE_A, 2, O>;
impl<'a, const O: u8> SIZE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(SIZE_A::_8BITS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(SIZE_A::_16BITS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut W {
        self.variant(SIZE_A::_32BITS)
    }
}
#[doc = "Field `MODE` reader - Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "1: `1`"]
    SLIDING = 1,
    #[doc = "0: `0`"]
    LINEAR = 0,
    #[doc = "2: `10`"]
    CIRCULAR = 2,
    #[doc = "3: `11`"]
    _2D = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            1 => MODE_A::SLIDING,
            0 => MODE_A::LINEAR,
            2 => MODE_A::CIRCULAR,
            3 => MODE_A::_2D,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLIDING`"]
    #[inline(always)]
    pub fn is_sliding(&self) -> bool {
        *self == MODE_A::SLIDING
    }
    #[doc = "Checks if the value of the field is `LINEAR`"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == MODE_A::LINEAR
    }
    #[doc = "Checks if the value of the field is `CIRCULAR`"]
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        *self == MODE_A::CIRCULAR
    }
    #[doc = "Checks if the value of the field is `_2D`"]
    #[inline(always)]
    pub fn is_2d(&self) -> bool {
        *self == MODE_A::_2D
    }
}
#[doc = "Field `MODE` writer - Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D"]
pub type MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RX_CH_CFG_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sliding(self) -> &'a mut W {
        self.variant(MODE_A::SLIDING)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn linear(self) -> &'a mut W {
        self.variant(MODE_A::LINEAR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn circular(self) -> &'a mut W {
        self.variant(MODE_A::CIRCULAR)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _2d(self) -> &'a mut W {
        self.variant(MODE_A::_2D)
    }
}
impl R {
    #[doc = "Bits 0:1 - Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data transfer format: - 2'b00: 8-bit - 2'b01:16-bit - 2;b10:32-bit"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<0> {
        SIZE_W::new(self)
    }
    #[doc = "Bits 8:9 - Data transfer mode: - 2'b00: Linear - 2'b01: Sliding - 2;b10:Circular - 2;b11:2D"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER RX channel configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ch_cfg](index.html) module"]
pub struct RX_CH_CFG_SPEC;
impl crate::RegisterSpec for RX_CH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ch_cfg::R](R) reader structure"]
impl crate::Readable for RX_CH_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ch_cfg::W](W) writer structure"]
impl crate::Writable for RX_CH_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CH_CFG to value 0"]
impl crate::Resettable for RX_CH_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
