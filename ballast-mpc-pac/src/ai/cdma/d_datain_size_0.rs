#[doc = "Register `D_DATAIN_SIZE_0` reader"]
pub struct R(crate::R<D_DATAIN_SIZE_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATAIN_SIZE_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATAIN_SIZE_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATAIN_SIZE_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATAIN_WIDTH` reader - "]
pub struct DATAIN_WIDTH_R(crate::FieldReader<u16, u16>);
impl DATAIN_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DATAIN_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAIN_WIDTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAIN_HEIGHT` reader - "]
pub struct DATAIN_HEIGHT_R(crate::FieldReader<u16, u16>);
impl DATAIN_HEIGHT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DATAIN_HEIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAIN_HEIGHT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn datain_width(&self) -> DATAIN_WIDTH_R {
        DATAIN_WIDTH_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn datain_height(&self) -> DATAIN_HEIGHT_R {
        DATAIN_HEIGHT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "Input cube’s width and height\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_datain_size_0](index.html) module"]
pub struct D_DATAIN_SIZE_0_SPEC;
impl crate::RegisterSpec for D_DATAIN_SIZE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_datain_size_0::R](R) reader structure"]
impl crate::Readable for D_DATAIN_SIZE_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DATAIN_SIZE_0 to value 0"]
impl crate::Resettable for D_DATAIN_SIZE_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
