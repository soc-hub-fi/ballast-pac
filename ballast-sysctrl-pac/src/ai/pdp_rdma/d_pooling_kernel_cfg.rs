#[doc = "Register `D_POOLING_KERNEL_CFG` reader"]
pub struct R(crate::R<D_POOLING_KERNEL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_POOLING_KERNEL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_POOLING_KERNEL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_POOLING_KERNEL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_POOLING_KERNEL_CFG` writer"]
pub struct W(crate::W<D_POOLING_KERNEL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_POOLING_KERNEL_CFG_SPEC>;
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
impl From<crate::W<D_POOLING_KERNEL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_POOLING_KERNEL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KERNEL_WIDTH` reader - "]
pub type KERNEL_WIDTH_R = crate::FieldReader<u8, KERNEL_WIDTH_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KERNEL_WIDTH_A {
    #[doc = "0: `0`"]
    WIDTH_1 = 0,
    #[doc = "1: `1`"]
    WIDTH_2 = 1,
    #[doc = "2: `10`"]
    WIDTH_3 = 2,
    #[doc = "3: `11`"]
    WIDTH_4 = 3,
    #[doc = "4: `100`"]
    WIDTH_5 = 4,
    #[doc = "5: `101`"]
    WIDTH_6 = 5,
    #[doc = "6: `110`"]
    WIDTH_7 = 6,
    #[doc = "7: `111`"]
    WIDTH_8 = 7,
}
impl From<KERNEL_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: KERNEL_WIDTH_A) -> Self {
        variant as _
    }
}
impl KERNEL_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KERNEL_WIDTH_A> {
        match self.bits {
            0 => Some(KERNEL_WIDTH_A::WIDTH_1),
            1 => Some(KERNEL_WIDTH_A::WIDTH_2),
            2 => Some(KERNEL_WIDTH_A::WIDTH_3),
            3 => Some(KERNEL_WIDTH_A::WIDTH_4),
            4 => Some(KERNEL_WIDTH_A::WIDTH_5),
            5 => Some(KERNEL_WIDTH_A::WIDTH_6),
            6 => Some(KERNEL_WIDTH_A::WIDTH_7),
            7 => Some(KERNEL_WIDTH_A::WIDTH_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WIDTH_1`"]
    #[inline(always)]
    pub fn is_width_1(&self) -> bool {
        *self == KERNEL_WIDTH_A::WIDTH_1
    }
    #[doc = "Checks if the value of the field is `WIDTH_2`"]
    #[inline(always)]
    pub fn is_width_2(&self) -> bool {
        *self == KERNEL_WIDTH_A::WIDTH_2
    }
    #[doc = "Checks if the value of the field is `WIDTH_3`"]
    #[inline(always)]
    pub fn is_width_3(&self) -> bool {
        *self == KERNEL_WIDTH_A::WIDTH_3
    }
    #[doc = "Checks if the value of the field is `WIDTH_4`"]
    #[inline(always)]
    pub fn is_width_4(&self) -> bool {
        *self == KERNEL_WIDTH_A::WIDTH_4
    }
    #[doc = "Checks if the value of the field is `WIDTH_5`"]
    #[inline(always)]
    pub fn is_width_5(&self) -> bool {
        *self == KERNEL_WIDTH_A::WIDTH_5
    }
    #[doc = "Checks if the value of the field is `WIDTH_6`"]
    #[inline(always)]
    pub fn is_width_6(&self) -> bool {
        *self == KERNEL_WIDTH_A::WIDTH_6
    }
    #[doc = "Checks if the value of the field is `WIDTH_7`"]
    #[inline(always)]
    pub fn is_width_7(&self) -> bool {
        *self == KERNEL_WIDTH_A::WIDTH_7
    }
    #[doc = "Checks if the value of the field is `WIDTH_8`"]
    #[inline(always)]
    pub fn is_width_8(&self) -> bool {
        *self == KERNEL_WIDTH_A::WIDTH_8
    }
}
#[doc = "Field `KERNEL_WIDTH` writer - "]
pub type KERNEL_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_POOLING_KERNEL_CFG_SPEC, u8, KERNEL_WIDTH_A, 4, O>;
impl<'a, const O: u8> KERNEL_WIDTH_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn width_1(self) -> &'a mut W {
        self.variant(KERNEL_WIDTH_A::WIDTH_1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn width_2(self) -> &'a mut W {
        self.variant(KERNEL_WIDTH_A::WIDTH_2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn width_3(self) -> &'a mut W {
        self.variant(KERNEL_WIDTH_A::WIDTH_3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn width_4(self) -> &'a mut W {
        self.variant(KERNEL_WIDTH_A::WIDTH_4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn width_5(self) -> &'a mut W {
        self.variant(KERNEL_WIDTH_A::WIDTH_5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn width_6(self) -> &'a mut W {
        self.variant(KERNEL_WIDTH_A::WIDTH_6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn width_7(self) -> &'a mut W {
        self.variant(KERNEL_WIDTH_A::WIDTH_7)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn width_8(self) -> &'a mut W {
        self.variant(KERNEL_WIDTH_A::WIDTH_8)
    }
}
#[doc = "Field `KERNEL_STRIDE_WIDTH` reader - "]
pub type KERNEL_STRIDE_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KERNEL_STRIDE_WIDTH` writer - "]
pub type KERNEL_STRIDE_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_POOLING_KERNEL_CFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn kernel_width(&self) -> KERNEL_WIDTH_R {
        KERNEL_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn kernel_stride_width(&self) -> KERNEL_STRIDE_WIDTH_R {
        KERNEL_STRIDE_WIDTH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn kernel_width(&mut self) -> KERNEL_WIDTH_W<0> {
        KERNEL_WIDTH_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn kernel_stride_width(&mut self) -> KERNEL_STRIDE_WIDTH_W<4> {
        KERNEL_STRIDE_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pooling_kernel_cfg](index.html) module"]
pub struct D_POOLING_KERNEL_CFG_SPEC;
impl crate::RegisterSpec for D_POOLING_KERNEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pooling_kernel_cfg::R](R) reader structure"]
impl crate::Readable for D_POOLING_KERNEL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_pooling_kernel_cfg::W](W) writer structure"]
impl crate::Writable for D_POOLING_KERNEL_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_POOLING_KERNEL_CFG to value 0"]
impl crate::Resettable for D_POOLING_KERNEL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
