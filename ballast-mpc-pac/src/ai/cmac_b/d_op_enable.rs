#[doc = "Register `D_OP_ENABLE` reader"]
pub struct R(crate::R<D_OP_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_OP_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_OP_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_OP_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_OP_ENABLE` writer"]
pub struct W(crate::W<D_OP_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_OP_ENABLE_SPEC>;
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
impl From<crate::W<D_OP_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_OP_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OP_EN` reader - "]
pub type OP_EN_R = crate::BitReader<OP_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OP_EN_A {
    #[doc = "1: `1`"]
    ENABLE = 1,
    #[doc = "0: `0`"]
    DISABLE = 0,
}
impl From<OP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OP_EN_A {
        match self.bits {
            true => OP_EN_A::ENABLE,
            false => OP_EN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OP_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OP_EN_A::DISABLE
    }
}
#[doc = "Field `OP_EN` writer - "]
pub type OP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, D_OP_ENABLE_SPEC, OP_EN_A, O>;
impl<'a, const O: u8> OP_EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OP_EN_A::ENABLE)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OP_EN_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn op_en(&self) -> OP_EN_R {
        OP_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn op_en(&mut self) -> OP_EN_W<0> {
        OP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set it to 1 to kick off operation for current register group\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_op_enable](index.html) module"]
pub struct D_OP_ENABLE_SPEC;
impl crate::RegisterSpec for D_OP_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_op_enable::R](R) reader structure"]
impl crate::Readable for D_OP_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_op_enable::W](W) writer structure"]
impl crate::Writable for D_OP_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_OP_ENABLE to value 0"]
impl crate::Resettable for D_OP_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
