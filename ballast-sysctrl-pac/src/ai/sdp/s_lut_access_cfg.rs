#[doc = "Register `S_LUT_ACCESS_CFG` reader"]
pub struct R(crate::R<S_LUT_ACCESS_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_ACCESS_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_ACCESS_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_ACCESS_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_ACCESS_CFG` writer"]
pub struct W(crate::W<S_LUT_ACCESS_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_ACCESS_CFG_SPEC>;
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
impl From<crate::W<S_LUT_ACCESS_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_ACCESS_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_ADDR` reader - "]
pub type LUT_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LUT_ADDR` writer - "]
pub type LUT_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_ACCESS_CFG_SPEC, u16, u16, 10, O>;
#[doc = "Field `LUT_TABLE_ID` reader - "]
pub type LUT_TABLE_ID_R = crate::BitReader<LUT_TABLE_ID_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT_TABLE_ID_A {
    #[doc = "0: `0`"]
    LE = 0,
    #[doc = "1: `1`"]
    LO = 1,
}
impl From<LUT_TABLE_ID_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_TABLE_ID_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT_TABLE_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_TABLE_ID_A {
        match self.bits {
            false => LUT_TABLE_ID_A::LE,
            true => LUT_TABLE_ID_A::LO,
        }
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == LUT_TABLE_ID_A::LE
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == LUT_TABLE_ID_A::LO
    }
}
#[doc = "Field `LUT_TABLE_ID` writer - "]
pub type LUT_TABLE_ID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, S_LUT_ACCESS_CFG_SPEC, LUT_TABLE_ID_A, O>;
impl<'a, const O: u8> LUT_TABLE_ID_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(LUT_TABLE_ID_A::LE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(LUT_TABLE_ID_A::LO)
    }
}
#[doc = "Field `LUT_ACCESS_TYPE` reader - "]
pub type LUT_ACCESS_TYPE_R = crate::BitReader<LUT_ACCESS_TYPE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LUT_ACCESS_TYPE_A {
    #[doc = "0: `0`"]
    READ = 0,
    #[doc = "1: `1`"]
    WRITE = 1,
}
impl From<LUT_ACCESS_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: LUT_ACCESS_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl LUT_ACCESS_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LUT_ACCESS_TYPE_A {
        match self.bits {
            false => LUT_ACCESS_TYPE_A::READ,
            true => LUT_ACCESS_TYPE_A::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == LUT_ACCESS_TYPE_A::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == LUT_ACCESS_TYPE_A::WRITE
    }
}
#[doc = "Field `LUT_ACCESS_TYPE` writer - "]
pub type LUT_ACCESS_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, S_LUT_ACCESS_CFG_SPEC, LUT_ACCESS_TYPE_A, O>;
impl<'a, const O: u8> LUT_ACCESS_TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(LUT_ACCESS_TYPE_A::READ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(LUT_ACCESS_TYPE_A::WRITE)
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn lut_addr(&self) -> LUT_ADDR_R {
        LUT_ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lut_table_id(&self) -> LUT_TABLE_ID_R {
        LUT_TABLE_ID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn lut_access_type(&self) -> LUT_ACCESS_TYPE_R {
        LUT_ACCESS_TYPE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn lut_addr(&mut self) -> LUT_ADDR_W<0> {
        LUT_ADDR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn lut_table_id(&mut self) -> LUT_TABLE_ID_W<16> {
        LUT_TABLE_ID_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn lut_access_type(&mut self) -> LUT_ACCESS_TYPE_W<17> {
        LUT_ACCESS_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT access address and type\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_access_cfg](index.html) module"]
pub struct S_LUT_ACCESS_CFG_SPEC;
impl crate::RegisterSpec for S_LUT_ACCESS_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_access_cfg::R](R) reader structure"]
impl crate::Readable for S_LUT_ACCESS_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_access_cfg::W](W) writer structure"]
impl crate::Writable for S_LUT_ACCESS_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S_LUT_ACCESS_CFG to value 0"]
impl crate::Resettable for S_LUT_ACCESS_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
